use crate::utils::r#ref::RefMut;
use ahash::{AHasher, RandomState};
use core::hash::{BuildHasher, Hash, Hasher};
use hashbrown::HashMap;
use lazy_static::*;
use std::sync::Mutex;


#[allow(dead_code)]
lazy_static! {
    static ref B_HASHER: AHasher = RandomState::new().build_hasher();
}
fn shard_amount() -> usize {
    (num_cpus::get() * 4).next_power_of_two()
}

fn ncb(shard_amount: usize) -> usize {
    shard_amount.trailing_zeros() as usize
}

#[derive(Clone,Copy)]
pub enum UpdateType {
    LAZY,
    COPY,
}

pub struct DBDashMap<K, V, S = RandomState> {
    shift: usize,
    pub shards: Box<[Mutex<HashMap<K, V, S>>]>,
    pub r_shards: Box<[HashMap<K, V, S>]>,
    hasher: S,
    pub update_type: Mutex<UpdateType>,
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
            update_type: Mutex::new(UpdateType::LAZY),
        }
    }

    pub fn hash_usize<T: Hash>(&self, item: &T) -> usize {
        // let mut hasher = self.hasher.build_hasher();
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

        let mut shard = self.shards[idx].lock().unwrap();

        shard.insert(key, value)
    }

    pub fn remove(&self, key: &K) -> Option<(K, V)> {
        let hash = self.hash_usize(&key);

        let idx = self.determine_shard(hash);

        let mut shard = self.shards[idx].lock().unwrap();

        shard.remove_entry(key)
    }

    pub fn get(&'a self, key: &K) -> Option<&'a V> {
        let hash = self.hash_usize(&key);

        let idx = self.determine_shard(hash);

        let shard = &self.r_shards[idx];

        shard.get(key)
    }

    pub fn get_mut(&'a self, key: &K) -> Option<RefMut<K, V, S>> {
        let hash = self.hash_usize(&key);

        let idx = self.determine_shard(hash);

        let mut shard = self.shards[idx].lock().unwrap();

        match shard.get_mut(key) {
            Some(r) => unsafe {
                let re = &mut *(r as *mut V);
                Some(RefMut::new(shard, re))
            },
            None => None,
        }
    }

    pub fn lazy_update(&self) {
        let shard_amount = shard_amount();
        for i in 0..shard_amount {
            unsafe {
                std::ptr::swap(
                    &*self.shards[i].lock().unwrap() as *const HashMap<K,V,S> as *mut HashMap<K, V, S>,
                    &self.r_shards[i] as *const HashMap<K, V, S> as *mut HashMap<K,V,S>,
                )
            }
            self.shards[i].lock().unwrap().clear();
        }
        *self.update_type.lock().unwrap() = UpdateType::LAZY;
    }

    pub fn merge_r_shards(&mut self) -> HashMap<K, V, S> {
        let mut ris = HashMap::with_hasher(self.hasher.clone());
        for i in 0..shard_amount() {
            ris.extend(self.shards[i].get_mut().unwrap().drain());
        }
        ris
    }

    pub fn len(&self) -> usize {
        self.shards
            .iter()
            .map(|shard| shard.lock().unwrap().len())
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

    

    pub fn clear(&self){
        for shard in self.shards.iter(){
            shard.lock().unwrap().clear();
        }
    }

    pub fn is_empty(&self) -> bool{
        self.len() == 0
    }

    pub fn is_empty_r(&self) -> bool{
        self.r_len() == 0
    }
}

impl<'a, K: 'a + Eq + Hash + Clone, V: Clone + 'a> DBDashMap<K, V, RandomState> {
    pub fn update(&self) {
        let n = shard_amount();
        unsafe{
        for i in 0..n {
            let r_shard = &self.r_shards[i] as *const HashMap<K,V,RandomState> as *mut HashMap<K,V,RandomState>;
            r_shard.as_mut().unwrap().clear();
            for (key, value) in self.shards[i].lock().unwrap().iter() {
                r_shard.as_mut().unwrap().insert(key.clone(), value.clone());
            }
        }
        *self.update_type.lock().unwrap() = UpdateType::COPY;
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

    pub fn w_keys(&self) -> Vec<K> {
        let mut ris: Vec<K> = Vec::with_capacity(self.len());
        for shard in self.shards.iter() {
            for key in shard.lock().unwrap().keys() {
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
