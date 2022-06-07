# Timed Set
[![Linux Arm7](https://github.com/marirs/timed-set-rs/actions/workflows/linux_arm.yml/badge.svg)](https://github.com/marirs/timed-set-rs/actions/workflows/linux_arm.yml)
[![Linux x86_64](https://github.com/marirs/timed-set-rs/actions/workflows/linux_intel.yml/badge.svg)](https://github.com/marirs/timed-set-rs/actions/workflows/linux_intel.yml)
[![macOS intel](https://github.com/marirs/timed-set-rs/actions/workflows/macos_intel.yml/badge.svg)](https://github.com/marirs/timed-set-rs/actions/workflows/macos_intel.yml)
[![Windows](https://github.com/marirs/timed-set-rs/actions/workflows/win_intel.yml/badge.svg)](https://github.com/marirs/timed-set-rs/actions/workflows/win_intel.yml)

A simple timed set in Rust to store elements for a given time period.

### Usage

```toml
[dependencies]
timed_set = "0.0.4"
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

Custom `ttl` for specifically 1 element
```rust
use timed_set::TimedSet;
use std::{time::Duration, thread::sleep};

fn main() {
    let mut ts = TimedSet::new(Duration::from_secs(3));
    ts.add("element_1");
    ts.add("element_2", Duration::from_secs(10));   // element with custom ttl
    assert!(ts.contains(&"element_1"));
    assert!(ts.contains(&"element_2"));
    
    sleep(Duration::from_secs(3));
    assert!(!ts.contains(&"element_1"));    // expired
    assert!(ts.contains(&"element_2"));
    
    sleep(Duration::from_secs(8));
    assert!(!ts.contains(&"element_2"));    // expired
}
```

Using it with lazy_static
```rust
use timed_set::TimedSet;
use std::{time::Duration, thread::sleep, sync::Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref TS: Mutex<TimedSet<String>> = Mutex::new(TimedSet::new(Duration::from_secs(3)));
}

fn main() {
    TS.lock().unwrap().add("element_1".to_string());
    TS.lock().unwrap().add("element_2".to_string(), Duration::from_secs(10));   // element with custom ttl
    assert!(TS.lock().unwrap().contains(&"element_1".to_string()));
    assert!(TS.lock().unwrap().contains(&"element_2".to_string()));
    
    sleep(Duration::from_secs(3));
    assert!(!TS.lock().unwrap().contains(&"element_1".to_string()));    // expired
    assert!(TS.lock().unwrap().contains(&"element_2".to_string()));
    
    sleep(Duration::from_secs(8));
    assert!(!TS.lock().unwrap().contains(&"element_2".to_string()));    // expired
}
```

---
License: MIT
