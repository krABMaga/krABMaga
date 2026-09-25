use krabmaga::engine::fields::field::Field;

#[derive(Default)]
struct DefaultField {
    updates: usize,
    lazy_updates: usize,
}

impl Field for DefaultField {}

#[test]
fn field_default_methods_are_noop() {
    let mut field = DefaultField::default();

    field.update();
    field.lazy_update();

    assert_eq!(field.updates, 0);
    assert_eq!(field.lazy_updates, 0);
}
