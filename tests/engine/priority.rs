#[test]
fn priority_orders_by_time_then_ordering() {
    use std::cmp::Ordering;

    use krabmaga::engine::priority::Priority;

    let earlier = Priority::new(1.0, 0);
    let later = Priority::new(2.0, 0);
    let same_time_lower_ordering = Priority::new(1.0, 0);
    let same_time_higher_ordering = Priority::new(1.0, 1);

    assert_eq!(earlier.cmp(&later), Ordering::Greater);
    assert_eq!(later.cmp(&earlier), Ordering::Less);
    assert_eq!(
        same_time_lower_ordering.cmp(&same_time_higher_ordering),
        Ordering::Greater
    );
    assert_eq!(
        same_time_higher_ordering.cmp(&same_time_lower_ordering),
        Ordering::Less
    );
    assert!(same_time_lower_ordering == Priority::new(1.0, 0));
    assert_eq!(same_time_lower_ordering.to_string(), "1 0");
}
