use std::sync::{RwLockReadGuard,Arc};
use std::collections::HashMap;
use crate::location::Int2D;

pub struct Ref<'a,V> {
    guard: Option<Arc<RwLockReadGuard<'a,HashMap<Int2D, Vec<V>>>>>,
    v: &'a V,
}

unsafe impl<'a, V: Send> Send for Ref<'a, V> {}

unsafe impl<'a, V: Send + Sync> Sync
    for Ref<'a,  V>
{
}

impl<'a, V> Ref<'a, V> {
    pub(crate) fn new(guard: Option<Arc<RwLockReadGuard<'a,HashMap<Int2D, Vec<V>>>>>, v: &'a V) -> Self {
        Self {
            guard,
            v,
        }
    }

    pub fn value(&self) -> &V {
        self.v
    }

}

use std::ops::Deref;
impl<'a,V> Deref for Ref<'a, V> {
    type Target = V;

    fn deref(&self) -> &V {
        self.value()
    }
}
