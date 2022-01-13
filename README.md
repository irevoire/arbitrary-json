Arbitrary JSON
==============

This crate provide a compatibility layer between
[serde_json](https://github.com/serde-rs/json) and
[arbitrary](https://github.com/rust-fuzz/arbitrary).
This allow you to generate random valid json when fuzzing your rust code. See
the following example:

```rust
#![no_main]
use arbitrary_json::ArbitraryValue;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: ArbitraryValue| {
    // call your very complex code here
    if data["truc"] == serde_json::json!(42) {
        panic!("Found the magic value");
    }
});
```