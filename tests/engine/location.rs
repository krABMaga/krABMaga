#[test]
fn location_display_and_equality() {
    use std::collections::HashSet;

    use krabmaga::engine::location::{Int2D, Real2D};

    let real = Real2D { x: 1.5, y: -2.25 };
    assert_eq!(real.to_string(), "1.5 -2.25");
    assert!(real == Real2D { x: 1.5, y: -2.25 });
    assert!(real != Real2D { x: 1.5, y: 2.25 });

    let int = Int2D { x: 3, y: 7 };
    assert_eq!(int.to_string(), "3 7");
    assert!(int == Int2D { x: 3, y: 7 });
    assert!(int != Int2D { x: 7, y: 3 });

    let mut locations = HashSet::new();
    locations.insert(int);
    assert!(locations.contains(&Int2D { x: 3, y: 7 }));
}
