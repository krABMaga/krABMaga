use std::cmp::{Eq, Ordering};
use std::fmt;

#[derive(Clone)]
pub struct Priority {
    pub time: f32,
    pub ordering: i32,
}

impl Priority {
    pub fn new(the_time: f32, the_ordering: i32) -> Priority {
        Priority {
            time: the_time,
            ordering: the_ordering,
        }
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Priority) -> Ordering {
        if self.time < other.time {
            return Ordering::Greater;
        }
        if self.time > other.time {
            return Ordering::Less;
        }
        if self.ordering < other.ordering {
            return Ordering::Greater;
        }
        if self.ordering > other.ordering {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

impl PartialOrd for Priority {
    fn partial_cmp(&self, other: &Priority) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Priority {}

impl PartialEq for Priority {
    fn eq(&self, other: &Priority) -> bool {
        self.ordering == other.ordering && self.time == other.time
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.time, self.ordering)
    }
}
