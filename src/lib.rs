use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

pub struct TimedSet<T> {
    ttl: Duration,
    set: HashMap<T, SystemTime>,
}

impl<T> TimedSet<T>
where
    T: std::hash::Hash + Eq,
{
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            set: HashMap::new(),
        }
    }

    pub fn add(&mut self, val: T) {
        self.set.insert(val, SystemTime::now() + self.ttl);
    }

    pub fn contains(&self, val: &T) -> bool {
        if let Some(s) = self.set.get(val) {
            if SystemTime::now() < *s {
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            set: self.set.iter().map(|(k, v)| (k, v)).collect(),
        }
    }
}

pub struct Iter<'a, T> {
    set: HashMap<&'a T, &'a SystemTime>,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Copy + std::hash::Hash + Eq,
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let keys: Vec<&T> = self.set.keys().cloned().collect();
        for k in keys {
            if let Some((v, t)) = self.set.remove_entry(&k) {
                if SystemTime::now() < *t {
                    return Some(v);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timedset() {
        // Create the Timed set with a TTL of 10 seconds
        let mut ts = TimedSet::new(Duration::from_secs(10));
        // add elements into the map
        ts.add("test1");
        ts.add("test2");
        // check if the elements are present
        assert!(ts.contains(&"test1"));
        assert!(ts.contains(&"test2"));
        // wait for 5 seconds
        std::thread::sleep(Duration::from_secs(5));
        // check if elements are present
        assert!(ts.contains(&"test1"));
        assert!(ts.contains(&"test2"));
        // wait for another 5 seconds
        std::thread::sleep(Duration::from_secs(5));
        // check if elements are not present now, as they should have got expired
        assert!(!ts.contains(&"test1"));
        assert!(!ts.contains(&"test2"));
    }
}
