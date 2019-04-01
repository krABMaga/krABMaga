use std::cmp::Ordering;
use std::cmp::Eq;

pub struct Priority{
    pub time:f64,
    pub priority:i64
}

impl Ord for Priority {
    fn cmp(&self, other: &Priority) -> Ordering {
        if self.priority < other.priority {return Ordering::Less;}
        if self.priority > other.priority {return Ordering::Greater;}
        //return self.time.cmp(&other.time)
        if self.time < other.time {return Ordering::Less;}
        if self.time > other.time {return Ordering::Greater;}

        return Ordering::Equal
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
        self.priority == other.priority && self.time == other.time
    }
}
