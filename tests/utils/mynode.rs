use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug)]
pub struct MyNode {
    pub id: u32,
    pub flag: bool,
}

impl Eq for MyNode {}

impl PartialEq for MyNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for MyNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} flag {}", self.id, self.flag)
    }
}

impl Hash for MyNode {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.id.hash(state);
    }
}
