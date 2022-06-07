use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

/// A TimedSet that keeps a TTL for each of its elements.
/// After the time expires, the elements are removed.
/// ## Example
/// ```rust
/// use timed_set::TimedSet;
/// use std::{time::Duration, thread::sleep};
///
/// let mut ts = TimedSet::new();
/// ts.add("element_1", Duration::from_secs(1));
/// assert!(ts.contains(&"element_1"));
/// sleep(Duration::from_secs(1));
/// assert!(!ts.contains(&"element_1"));
/// ```
pub struct TimedSet<T> {
    set: HashMap<T, SystemTime>,
}

impl<T> TimedSet<T>
where
    T: std::hash::Hash + Eq,
{
    /// Create a new TimedSet with a TTL of its elements
    /// ## Example
    /// ```rust
    /// use timed_set::TimedSet;
    ///
    /// let mut ts: TimedSet<&str> = TimedSet::new();
    ///
    /// ```
    pub fn new() -> Self {
        Self {
            set: HashMap::new(),
        }
    }

    /// Add/Insert an element into the timed set
    /// ## Example
    /// ```rust
    /// use timed_set::TimedSet;
    /// use std::time::Duration;
    ///
    /// let mut ts = TimedSet::new();
    /// ts.add("element1", Duration::from_secs(2));
    /// ```
    pub fn add(&mut self, val: T, ttl: Duration) {
        self.set.insert(val, SystemTime::now() + ttl);
    }

    /// Check if an element is present in the TimedSet
    /// ## Example
    /// ```rust
    /// use timed_set::TimedSet;
    /// use std::time::Duration;
    ///
    /// let mut ts = TimedSet::new();
    /// ts.add("element1", Duration::from_secs(2));
    /// assert!(ts.contains(&"element1"));
    /// ```
    pub fn contains(&self, val: &T) -> bool {
        if let Some(s) = self.set.get(val) {
            if SystemTime::now() < *s {
                return true;
            }
        }
        false
    }

    /// Iterator
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            set: self.set.iter().map(|(k, v)| (k, v)).collect(),
        }
    }
}

/// Iterator
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
    fn test_timedset_str() {
        // Create the Timed set with a TTL of 10 seconds
        let mut ts = TimedSet::new();
        // add elements into the map
        ts.add("element_1", Duration::from_secs(2));
        ts.add("element_2", Duration::from_secs(4));
        // check if the elements are present
        assert!(ts.contains(&"element_1"));
        assert!(ts.contains(&"element_2"));
        // wait for 5 seconds
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are present
        assert!(ts.contains(&"element_1"));
        assert!(ts.contains(&"element_2"));
        // wait for another 5 seconds
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are not present now, as they should have got expired
        assert!(!ts.contains(&"element_1"));
        assert!(ts.contains(&"element_2"));
        std::thread::sleep(Duration::from_secs(2));
        // check if elements are not present now, as they should have got expired
        assert!(!ts.contains(&"element_1"));
        assert!(!ts.contains(&"element_2"));
    }

    #[test]
    fn test_timedset_string() {
        // Create the Timed set with a TTL of 10 seconds
        let mut ts = TimedSet::new();
        // add elements into the map
        ts.add("element_1".to_string(), Duration::from_secs(2));
        ts.add("element_2".to_string(), Duration::from_secs(4));
        // check if the elements are present
        assert!(ts.contains(&"element_1".to_string()));
        assert!(ts.contains(&"element_2".to_string()));
        // wait for 5 seconds
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are present
        assert!(ts.contains(&"element_1".to_string()));
        assert!(ts.contains(&"element_2".to_string()));
        // wait for another 5 seconds
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are not present now, as they should have got expired
        assert!(!ts.contains(&"element_1".to_string()));
        assert!(ts.contains(&"element_2".to_string()));
        std::thread::sleep(Duration::from_secs(2));
        // check if elements are not present now, as they should have got expired
        assert!(!ts.contains(&"element_1".to_string()));
        assert!(!ts.contains(&"element_2".to_string()));
    }
}
