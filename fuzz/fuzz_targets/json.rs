#![no_main]
use arbitrary_json::ArbitraryValue;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: ArbitraryValue| {
    if data["truc"] == serde_json::json!(42) {
        panic!("Found the magic value");
    }
});
