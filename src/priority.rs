use std::cmp::Ordering;
use std::cmp::Eq;

pub struct Priority{
    pub time:f64,
    pub ordering:i64
}

impl Ord for Priority {
    fn cmp(&self, other: &Priority) -> Ordering {

        if self.time < other.time {return Ordering::Greater;}
        if self.time > other.time {return Ordering::Less;}

        if self.ordering < other.ordering {return Ordering::Greater;}
        if self.ordering > other.ordering {return Ordering::Less;}
        //return self.time.cmp(&other.time)

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
        self.ordering == other.ordering && self.time == other.time
    }
}
