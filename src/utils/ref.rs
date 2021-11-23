use ahash::RandomState;
use core::hash::{BuildHasher, Hash};
use core::ops::{Deref, DerefMut};
use hashbrown::HashMap;
use std::sync::MutexGuard;

#[allow(dead_code)]
pub struct RefMut<'a, K, V, S = RandomState> {
    guard: MutexGuard<'a, HashMap<K, V, S>>,
    v: &'a mut V,
}

unsafe impl<'a, K: Eq + Hash + Send, V: Send, S: BuildHasher> Send for RefMut<'a, K, V, S> {}

unsafe impl<'a, K: Eq + Hash + Send + Sync, V: Send + Sync, S: BuildHasher> Sync
    for RefMut<'a, K, V, S>
{
}

impl<'a, K: Eq + Hash, V, S: BuildHasher> RefMut<'a, K, V, S> {
    pub(crate) fn new(guard: MutexGuard<'a, HashMap<K, V, S>>, v: &'a mut V) -> Self {
        Self { guard, v }
    }

    pub fn value(&self) -> &V {
        self.v
    }

    pub fn value_mut(&mut self) -> &mut V {
        self.v
    }
}

impl<'a, K: Eq + Hash, V, S: BuildHasher> Deref for RefMut<'a, K, V, S> {
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}

impl<'a, K: Eq + Hash, V, S: BuildHasher> DerefMut for RefMut<'a, K, V, S> {
    fn deref_mut(&mut self) -> &mut V {
        self.value_mut()
    }
}
