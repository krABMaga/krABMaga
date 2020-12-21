use std::collections::HashMap;
use std::hash::Hash;
use std::hash::Hasher;

#[test]
fn hash_test_1() {
    let libro1 = Libro { id: 1, costo: 2.0 };
    let libro2 = Libro { id: 2, costo: 6.0 };
    //    let libro3 = Libro{id: 3, costo: 8.0};

    let mut map = HashMap::new();
    map.insert(libro1.clone(), 1);
    map.insert(libro2.clone(), 2);
    assert!(map.contains_key(&libro1.clone()));
    assert_eq!(1, *map.get(&libro1).unwrap());
    assert_eq!(2, map.len());

    map.insert(libro1.clone(), 4);
    assert_eq!(4, *map.get(&libro1).unwrap());

    assert_eq!(2, map.len());
}

#[derive(Debug, Clone)]
pub struct Libro {
    pub id: u64,
    pub costo: f64,
}

impl Hash for Libro {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u64(self.id);
        state.finish();
    }
}

impl Eq for Libro {}

impl PartialEq for Libro {
    fn eq(&self, other: &Libro) -> bool {
        self.id == other.id && self.costo == other.costo
    }
}
