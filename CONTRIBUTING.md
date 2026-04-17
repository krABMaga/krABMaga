# Contributing FAQ

Hello, thank you for contributing to this project! Here are some guidelines and helpful info that can help you during
the development phase.

The structure of the project is:

- engine: The krABMaga framework files are stored here, as well as data structures that can be used in simulation
  development.
- utils: Contains utilities used by the framework such as the DBDashMap data structure.
- visualization: Contains the (optional) visualization framework files that interact with krABMaga.

# prerequisites

```bash
cargo install --locked prek
prek install
```

## Commit message style

Commit messages should follow the [conventional commit specification](https://www.conventionalcommits.org/en/v1.0.0/).

The allowed structural elements are:
- `feat` for new features.
- `fix` for bug fixes.
- `chore` for changes to the build process or auxiliary tools and libraries such as documentation generation.
- `refactor` for code changes that neither fix a bug nor add a feature.
- `docs` for any documentation/README changes.

Commit messages should be structured in a way that can be read as if they were completing the sentence *"If applied, this commit will..."*. For example:

> feat: add new authentication method to API

Reads as *"If applied, this commit will add new authentication method to API"*.

## Branch naming

Branch names should follow the pattern `^(feat|fix|chore|refactor|docs)\/[a-z0-9]+(-[a-z0-9]+)*$`. This means that branch names should:
- Start with the same structural elements as commit messages.
- Be descriptive and contain only lowercase letters and numbers.
- Use hyphens to separate words.

For example:
- `feat/add-user-authentication`
- `fix/issue-with-database-connection`
- `chore/update-dependencies`
- `refactor/improve-code-structure`
- `docs/update-contributing-guidelines`

To verify that your branch name adheres to these guidelines, you can use the following command:

```bash
git rev-parse --abbrev-ref HEAD | grep -Eq '^(feat|fix|chore|refactor|docs)\/[a-z0-9]+(-[a-z0-9]+)*$' && echo "Branch name is compliant" || echo "Invalid branch name"
```

## Make utilities

Check [Makefile](Makefile) to see all pre-configured commands to run the library


# How to: Test

To test the framework's code, the command to run is `cargo test --lib`.

To test the rustdoc examples, `cargo test --doc`.

To run the integration tests located within the `tests` folder, `cargo test --tests`.

Finally, to run tests for a particular example, `cargo test --example exampleName`.

**WARNING:** By default, `cargo test` with no flags will try running all the tests and examples, including the ones related to the
visualization framework. This will lead to errors if the correct feature isn't passed, since it's required for those
examples to build successfully. If you want to test everything, simply add the visualization feature flag:

```sh
cargo test --features visualization
```
