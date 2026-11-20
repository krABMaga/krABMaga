use krabmaga::utils::dbdashmap::DBDashMap;

#[test]
fn refmut_allows_deref_and_mutation() {
    let mut map: DBDashMap<u32, u32> = DBDashMap::new();

    map.insert(1, 5);

    if let Some(mut entry) = map.get_write(&1) {
        assert_eq!(*entry, 5);
        *entry = 6;
        assert_eq!(*entry, 6);
    } else {
        panic!("Expected RefMut from get_write");
    }

    map.update();
    assert_eq!(map.get_read(&1), Some(&6));
}
