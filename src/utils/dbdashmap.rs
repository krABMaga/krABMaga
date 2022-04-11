use crate::utils::r#ref::RefMut;

use core::hash::{BuildHasher, Hash, Hasher};
use std::sync::Mutex;

use ahash::{AHasher, RandomState};
use hashbrown::HashMap;
use lazy_static::*;

//#[allow(dead_code)]
lazy_static! {
    static ref B_HASHER: AHasher = RandomState::new().build_hasher();
}
fn shard_amount() -> usize {
    (num_cpus::get() * 4).next_power_of_two()
}

fn ncb(shard_amount: usize) -> usize {
    shard_amount.trailing_zeros() as usize
}

pub struct DBDashMap<K, V, S = RandomState> {
    shift: usize,
    pub shards: Box<[Mutex<HashMap<K, V, S>>]>,
    pub r_shards: Box<[HashMap<K, V, S>]>,
    hasher: S,
}

impl<K, V, S> Default for DBDashMap<K, V, S>
where
    K: Eq + Hash,
    S: Default + BuildHasher + Clone,
{
    fn default() -> Self {
        Self::with_hasher(Default::default())
    }
}

impl<'a, K: 'a + Eq + Hash, V: 'a> DBDashMap<K, V, RandomState> {
    pub fn new() -> Self {
        DBDashMap::with_hasher(RandomState::default())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        DBDashMap::with_capacity_and_hasher(capacity, RandomState::default())
    }
}

impl<'a, K: 'a + Eq + Hash, V: 'a, S: BuildHasher + Clone + Default> DBDashMap<K, V, S> {
    pub fn with_hasher(hasher: S) -> Self {
        Self::with_capacity_and_hasher(0, hasher)
    }

    pub fn with_capacity_and_hasher(mut capacity: usize, hasher: S) -> Self {
        let shard_amount = shard_amount();
        let shift = ptr_size_bits() - ncb(shard_amount);

        if capacity != 0 {
            capacity = (capacity + (shard_amount - 1)) & !(shard_amount - 1);
        }

        let cps = capacity / shard_amount;

        let shards = (0..shard_amount)
            .map(|_| Mutex::new(HashMap::with_capacity_and_hasher(cps, hasher.clone())))
            .collect();

        let r_shards = (0..shard_amount)
            .map(|_| HashMap::with_capacity_and_hasher(cps, hasher.clone()))
            .collect();

        Self {
            shift,
            shards,
            r_shards,
            hasher,
        }
    }

    pub fn hash_usize<T: Hash>(&self, item: &T) -> usize {
        let mut hasher = B_HASHER.clone();

        item.hash(&mut hasher);

        hasher.finish() as usize
    }

    pub fn determine_shard(&self, hash: usize) -> usize {
        (hash << 7) >> self.shift
    }

    pub fn insert(&self, key: K, value: V) -> Option<V> {
        let hash = self.hash_usize(&key);

        let idx = self.determine_shard(hash);

        let mut shard = self.shards[idx].lock().expect("error on lock");

        shard.remove(&key);
        shard.insert(key, value)
    }

    pub fn remove(&self, key: &K) -> Option<(K, V)> {
        let hash = self.hash_usize(&key);

        let idx = self.determine_shard(hash);

        let mut shard = self.shards[idx].lock().expect("error on lock");

        shard.remove_entry(key)
    }

    pub fn get_read(&'a self, key: &K) -> Option<&'a V> {
        let hash = self.hash_usize(&key);

        let idx = self.determine_shard(hash);

        let shard = &self.r_shards[idx];

        shard.get(key)
    }

    pub fn get_key_value(&'a self, key: &K) -> Option<(&K, &V)> {
        let hash = self.hash_usize(&key);

        let idx = self.determine_shard(hash);

        let shard = &self.r_shards[idx];

        shard.get_key_value(key)
    }

    pub fn get_write(&'a self, key: &K) -> Option<RefMut<K, V, S>> {
        let hash = self.hash_usize(&key);

        let idx = self.determine_shard(hash);

        let mut shard = self.shards[idx].lock().expect("error on lock");

        match shard.get_mut(key) {
            Some(r) => unsafe {
                let re = &mut *(r as *mut V);
                Some(RefMut::new(shard, re))
            },
            None => None,
        }
    }

    pub fn lazy_update(&mut self) {
        let shard_amount = shard_amount();
        for i in 0..shard_amount {
            unsafe {
                std::ptr::swap(
                    self.shards[i].get_mut().expect("error on get_mut") as *mut HashMap<K, V, S>,
                    &mut self.r_shards[i] as *mut HashMap<K, V, S>,
                )
            }
            self.shards[i].get_mut().expect("error on get_mut").clear();
        }
    }

    pub fn merge_r_shards(&mut self) -> HashMap<K, V, S> {
        let mut ris = HashMap::with_hasher(self.hasher.clone());
        for i in 0..shard_amount() {
            ris.extend(self.shards[i].get_mut().expect("error on get_mut").drain());
        }
        ris
    }

    pub fn len(&self) -> usize {
        self.shards
            .iter()
            .map(|shard| shard.lock().expect("error on lock").len())
            .sum()
    }

    pub fn r_len(&self) -> usize {
        self.r_shards.iter().map(|shard| shard.len()).sum()
    }

    pub fn keys(&self) -> Vec<&K> {
        let mut ris = vec![];
        for shard in self.r_shards.iter() {
            for key in shard.keys() {
                ris.push(key);
            }
        }
        ris
    }

    pub fn values(&self) -> Vec<&V> {
        let mut ris = vec![];
        for shard in self.r_shards.iter() {
            for value in shard.values() {
                ris.push(value);
            }
        }
        ris
    }

    pub fn clear(&self) {
        for shard in self.shards.iter() {
            shard.lock().expect("error on lock").clear();
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn is_empty_r(&self) -> bool {
        self.r_len() == 0
    }
}

impl<'a, K: 'a + Eq + Hash + Clone, V: Clone + 'a> DBDashMap<K, V, RandomState> {
    pub fn update(&mut self) {
        let n = shard_amount();
        for i in 0..n {
            for (key, value) in self.shards[i].lock().expect("error on lock").iter() {
                self.r_shards[i].remove(&key.clone());
                self.r_shards[i].insert(key.clone(), value.clone());
            }
        }
    }

    pub fn apply_to_all_values<F>(&self, closure: F)
    where
        F: Fn(&V) -> V,
    {
        for shard in self.r_shards.iter() {
            for (key, value) in shard.iter() {
                self.insert(key.clone(), closure(value));
            }
        }
    }
    pub fn apply_to_all_values_write<F>(&self, closure: F)
    where
        F: Fn(&V) -> V,
    {
        for shard in self.shards.iter() {
            for (_key, value) in shard.lock().expect("error on lock").iter_mut() {
                // let hash = self.hash_usize(&key);
                // let idx = self.determine_shard(hash);
                *value = closure(value);
            }
        }
    }
    pub fn apply_to_all_values_read_write<F>(&self, closure: F)
    where
        F: Fn(&V) -> V,
    {
        for shard_id in 0..self.r_shards.len() {
            for (key, value) in self.r_shards[shard_id].iter() {
                // let hash = self.hash_usize(&key);
                // let idx = self.determine_shard(hash);
                let mut locs = self.shards[shard_id].lock().expect("error on lock");
                if let Some(write_value) = locs.get_mut(key) {
                    *write_value = closure(write_value);
                } else {
                    locs.insert(key.clone(), closure(value));
                }
            }
        }
    }

    pub fn apply_to_all_keys<F>(&self, closure: F)
    where
        F: Fn(&V, &K) -> Option<K>,
    {
        for shard in self.r_shards.iter() {
            for (key, value) in shard.iter() {
                let result = closure(value, key);
                if let Some(x) = result {
                    self.remove(key);
                    self.insert(x, value.clone());
                }
            }
        }
    }

    pub fn w_keys(&self) -> Vec<K> {
        let mut ris: Vec<K> = Vec::with_capacity(self.len());
        for shard in self.shards.iter() {
            for key in shard.lock().expect("error on lock").keys() {
                let k = key.to_owned();
                ris.push(k);
            }
        }
        ris
    }
}

const fn ptr_size_bits() -> usize {
    std::mem::size_of::<usize>() * 8
}
