use krabmaga::utils::dbdashmap::DBDashMap;

#[test]
fn dbdashmap_insert_update_and_read() {
    let mut map: DBDashMap<u32, u32> = DBDashMap::new();

    assert!(map.is_empty());

    map.insert(1, 10);
    map.insert(2, 20);
    map.update();

    assert_eq!(map.len(), 2);
    assert_eq!(map.r_len(), 2);
    assert_eq!(map.get_read(&1), Some(&10));
    assert_eq!(map.get_key_value(&2).map(|(_, v)| *v), Some(20));
}

#[test]
fn dbdashmap_write_then_lazy_update() {
    let mut map: DBDashMap<u32, u32> = DBDashMap::new();

    map.insert(7, 70);
    map.insert(8, 80);
    assert_eq!(map.len(), 2);
    assert!(map.is_empty_r());

    map.lazy_update();

    assert_eq!(map.len(), 0);
    assert_eq!(map.r_len(), 2);
    assert_eq!(map.get_read(&7), Some(&70));
    assert_eq!(map.get_read(&8), Some(&80));
}

#[test]
fn dbdashmap_get_write_and_apply() {
    let mut map: DBDashMap<u32, u32> = DBDashMap::new();

    map.insert(1, 1);
    map.insert(2, 2);

    if let Some(mut value) = map.get_write(&1) {
        *value += 9;
    }

    map.update();
    assert_eq!(map.get_read(&1), Some(&10));

    map.apply_to_all_values(|v| v + 1);
    map.update();

    assert_eq!(map.get_read(&1), Some(&11));
    assert_eq!(map.get_read(&2), Some(&3));
}

#[test]
fn dbdashmap_remove_and_clear() {
    let mut map: DBDashMap<u32, u32> = DBDashMap::new();

    map.insert(3, 30);
    map.insert(4, 40);

    let removed = map.remove(&3);
    assert_eq!(removed.map(|(_, v)| v), Some(30));

    map.update();
    assert_eq!(map.get_read(&3), None);
    assert_eq!(map.get_read(&4), Some(&40));

    map.clear();
    assert!(map.is_empty());
}
