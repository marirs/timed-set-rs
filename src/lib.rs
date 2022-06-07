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
/// let mut ts = TimedSet::new(Duration::from_secs(1));
/// ts.add("element_1");
/// assert!(ts.contains(&"element_1"));
/// sleep(Duration::from_secs(1));
/// assert!(!ts.contains(&"element_1"));
/// ```
pub struct TimedSet<T> {
    ttl: Duration,
    set: HashMap<T, SystemTime>,
}

impl<T> TimedSet<T>
where
    T: std::hash::Hash + Eq,
{
    /// Create a new TimedSet with a TTL of its elements. Here all the elements added into
    /// this TimedSet will inherit the TTL specified while initiating the TimedSet.
    /// ## Example
    /// ```rust
    /// use timed_set::TimedSet;
    /// use std::time::Duration;
    ///
    /// let mut ts: TimedSet<&str> = TimedSet::new(Duration::from_secs(2));
    ///
    /// ```
    pub fn new(ttl: Duration) -> Self {
        Self {
            ttl,
            set: HashMap::new(),
        }
    }

    /// Add/Insert an element into the timed set
    /// ## Example
    /// ```rust
    /// use timed_set::TimedSet;
    /// use std::time::Duration;
    ///
    /// let mut ts = TimedSet::new(Duration::from_secs(2));
    /// ts.add("element1");
    /// ```
    pub fn add(&mut self, val: T) {
        self.set.insert(val, SystemTime::now() + self.ttl);
    }

    /// Add/Insert an element into the timed set
    /// ## Example
    /// ```rust
    /// use timed_set::TimedSet;
    /// use std::time::Duration;
    ///
    /// let mut ts = TimedSet::new(Duration::from_secs(2));
    /// ts.add_with_ttl("element1", Duration::from_secs(1));
    /// ```
    pub fn add_with_ttl(&mut self, val: T, ttl: Duration) {
        self.set.insert(val, SystemTime::now() + ttl);
    }

    /// Check if an element is present in the TimedSet
    /// ## Example
    /// ```rust
    /// use timed_set::TimedSet;
    /// use std::time::Duration;
    ///
    /// let mut ts = TimedSet::new(Duration::from_secs(2));
    /// ts.add("element1");
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
    fn test_timedset_with_ttl_str() {
        // Create the Timed set with a TTL of 2 seconds
        let mut ts = TimedSet::new(Duration::from_secs(2));
        // add elements into the map
        ts.add("element_1");
        ts.add("element_2");
        // check if the elements are present
        assert!(ts.contains(&"element_1"));
        assert!(ts.contains(&"element_2"));
        // wait for 1 seconds
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are present
        assert!(ts.contains(&"element_1"));
        assert!(ts.contains(&"element_2"));
        // wait for another 1 seconds
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are not present now, as they should have got expired
        assert!(!ts.contains(&"element_1"));
        assert!(!ts.contains(&"element_2"));
    }

    #[test]
    fn test_timedset_with_ttl_string() {
        // Create the Timed set with a TTL of 2 seconds
        let mut ts = TimedSet::new(Duration::from_secs(2));
        // add elements into the map
        ts.add("element_1".to_string());
        ts.add("element_2".to_string());
        // check if the elements are present
        assert!(ts.contains(&"element_1".to_string()));
        assert!(ts.contains(&"element_2".to_string()));
        // wait for 1 seconds
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are present
        assert!(ts.contains(&"element_1".to_string()));
        assert!(ts.contains(&"element_2".to_string()));
        // wait for another 1 seconds
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are not present now, as they should have got expired
        assert!(!ts.contains(&"element_1".to_string()));
        assert!(!ts.contains(&"element_2".to_string()));
    }

    #[test]
    fn test_timedset_str() {
        // Create the Timed set
        let mut ts = TimedSet::new(Duration::from_secs(0));
        // add elements into the map with a ttl for the same
        ts.add_with_ttl("element_1", Duration::from_secs(2));
        ts.add_with_ttl("element_2", Duration::from_secs(3));
        // check if the elements are present
        assert!(ts.contains(&"element_1"));
        assert!(ts.contains(&"element_2"));
        // wait for 1 seconds
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are present
        assert!(ts.contains(&"element_1"));
        assert!(ts.contains(&"element_2"));
        // wait for another 1 seconds
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are not present now, as they should have got expired
        assert!(!ts.contains(&"element_1"));
        assert!(ts.contains(&"element_2"));
        std::thread::sleep(Duration::from_secs(1));
        // check if elements are not present now, as they should have got expired
        assert!(!ts.contains(&"element_1"));
        assert!(!ts.contains(&"element_2"));
    }

    #[test]
    fn test_timedset_mix() {
        // create a timed set with default 10 seconds ttl
        let mut ts = TimedSet::new(Duration::from_secs(2));
        // add element
        ts.add("element_1");
        // add element with a custom ttl
        ts.add_with_ttl("element_2", Duration::from_secs(3));
        // check to see if elements are there
        assert!(ts.contains(&"element_1"));
        assert!(ts.contains(&"element_2"));
        // wait for 1 second
        std::thread::sleep(Duration::from_secs(1));
        assert!(ts.contains(&"element_1"));
        assert!(ts.contains(&"element_2"));
        // wait for another second;
        std::thread::sleep(Duration::from_secs(1));
        assert!(!ts.contains(&"element_1"));    // should not be there as it expired
        assert!(ts.contains(&"element_2"));     // continues to be there as its with custom ttl
        // wait for 1 second
        std::thread::sleep(Duration::from_secs(1));
        assert!(!ts.contains(&"element_1"));    // expired
        assert!(!ts.contains(&"element_2"));     // expired
    }
}
