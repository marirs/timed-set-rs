# Timed Set
[![Linux Arm7](https://github.com/marirs/timed-set-rs/actions/workflows/linux_arm.yml/badge.svg)](https://github.com/marirs/timed-set-rs/actions/workflows/linux_arm.yml)
[![Linux x86_64](https://github.com/marirs/timed-set-rs/actions/workflows/linux_intel.yml/badge.svg)](https://github.com/marirs/timed-set-rs/actions/workflows/linux_intel.yml)
[![macOS intel](https://github.com/marirs/timed-set-rs/actions/workflows/macos_intel.yml/badge.svg)](https://github.com/marirs/timed-set-rs/actions/workflows/macos_intel.yml)
[![Windows](https://github.com/marirs/timed-set-rs/actions/workflows/win_intel.yml/badge.svg)](https://github.com/marirs/timed-set-rs/actions/workflows/win_intel.yml)

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