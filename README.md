# Timed Set

A simple timed set in Rust to store elements for a given time period.

### Usage

```toml
[dependencies]
timed_set = "0.0.1"
```

### Example

```rust
use timed_set::TimedSet;
use std::{time::Duration, thread::sleep};

fn main() {
    let mut ts = TimedSet::new(Duration::from_secs(3));
    ts.add("element_1");
    assert!(ts.contains(&"element_1"));
    sleep(Duration::from_secs(3));
    assert!(!ts.contains(&"element_1"));
}
```

---
License: MIT