use std::collections::HashSet;

#[test]
fn grid_option_is_hashable_and_distinct() {
    use krabmaga::engine::fields::grid_option::GridOption;

    let mut options = HashSet::new();
    options.insert(GridOption::READ);
    options.insert(GridOption::WRITE);
    options.insert(GridOption::READWRITE);

    assert_eq!(options.len(), 3);
    assert!(options.contains(&GridOption::READ));
    assert!(options.contains(&GridOption::WRITE));
    assert!(options.contains(&GridOption::READWRITE));
}
