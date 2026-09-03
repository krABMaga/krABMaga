#!/usr/bin/env bash

set -uo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${script_dir}/.." && pwd)"
workflows_dir="${repo_root}/.github/workflows"

# Print one consistently aligned row in the results table.
print_table_row() {
    printf '| %-6s | %-35s | %-12s | %-40s | %-34s |\n' \
        "$1" "$2" "$3" "$4" "$5"
}

# Ensure every command used by this script is available.
for command in git rg sed sort; do
    if ! command -v "${command}" >/dev/null 2>&1; then
        printf 'Required command not found: %s\n' "${command}" >&2
        exit 2
    fi
done

# Extract and deduplicate SHA-pinned actions and their version comments.
if ! pinned_actions="$(
    rg -o --no-filename \
        'uses:[[:space:]]*[^@[:space:]]+@[0-9a-f]{40}([[:space:]]+#[[:space:]]+[^[:space:]]+)?' \
        "${workflows_dir}" |
        sed -E 's/^uses:[[:space:]]*//' |
        sort -u
)"; then
    printf 'No SHA-pinned actions found in %s.\n' "${workflows_dir}" >&2
    exit 1
fi

# Track how many pins were checked and how many checks failed.
checked=0
failures=0

# Print the table header before checking the collected pins.
print_table_row "STATUS" "ACTION" "VERSION" "COMMIT SHA" "DETAILS"
print_table_row "------" "------" "-------" "----------" "-------"

# Validate the version comment and remote Git tag for each unique pin.
while read -r pinned marker version; do
    # Split the workflow reference into its action, commit, and repository parts.
    action="${pinned%@*}"
    sha="${pinned#*@}"
    owner="${action%%/*}"
    repo_and_path="${action#*/}"
    repository="${owner}/${repo_and_path%%/*}"

    # Require an exact semantic-version comment such as "# v1.2.3".
    if [[ "${marker:-}" != "#" || ! "${version:-}" =~ ^v[0-9]+\.[0-9]+\.[0-9]+([.-][0-9A-Za-z.-]+)?$ ]]; then
        print_table_row \
            "FAIL" "${action}" "${version:--}" "${sha}" \
            "Missing or invalid version comment"
        failures=$((failures + 1))
        continue
    fi

    # Query both lightweight and annotated forms of the version tag.
    checked=$((checked + 1))
    if ! refs="$(
        git ls-remote --tags "https://github.com/${repository}.git" \
            "refs/tags/${version}" "refs/tags/${version}^{}" 2>/dev/null
    )"; then
        print_table_row \
            "FAIL" "${action}" "${version}" "${sha}" \
            "Could not query repository"
        failures=$((failures + 1))
        continue
    fi

    # Check whether either tag form resolves to the pinned commit.
    tag_matches_pin=false
    while read -r tag_sha _; do
        if [[ "${tag_sha}" == "${sha}" ]]; then
            tag_matches_pin=true
            break
        fi
    done <<< "${refs}"

    # Report the result for the current action and record mismatches.
    if [[ "${tag_matches_pin}" == true ]]; then
        print_table_row "OK" "${action}" "${version}" "${sha}" "Matches tag"
    else
        print_table_row \
            "FAIL" "${action}" "${version}" "${sha}" \
            "Tag does not resolve to commit"
        failures=$((failures + 1))
    fi
done <<< "${pinned_actions}"

# Return a failing status when any pin could not be verified.
if ((failures > 0)); then
    printf '%d action version check(s) failed.\n' "${failures}" >&2
    exit 1
fi

# Summarize a fully successful verification run.
printf 'Verified %d pinned action version(s).\n' "${checked}"
