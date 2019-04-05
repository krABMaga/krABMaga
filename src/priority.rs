use std::cmp::Ordering;
use std::cmp::Eq;
use std::fmt;

#[derive(Clone)]
pub struct Priority{
    pub time:f64,
    pub ordering:i64
}

impl Priority {
    pub fn new(the_time: f64, the_ordering: i64) -> Priority {
        Priority {
            time: the_time,
            ordering: the_ordering
        }
    }
}

impl Ord for Priority {
    fn cmp(&self, other: &Priority) -> Ordering {

        if self.time < other.time {return Ordering::Greater;}
        if self.time > other.time {return Ordering::Less;}

        if self.ordering < other.ordering {return Ordering::Greater;}
        if self.ordering > other.ordering {return Ordering::Less;}
        //return self.time.cmp(&other.time)

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
