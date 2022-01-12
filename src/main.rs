use std::io::Read;

use arbitrary_json::ArbitraryValue;
use serde_json::Value;

fn main() {
    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();

    let mut unstructured = arbitrary::Unstructured::new(&input);
    let json: ArbitraryValue = unstructured.arbitrary().unwrap();
    let json: Value = json.into();
    let json = serde_json::to_string_pretty(&json).unwrap_or_else(|_| json.to_string());

    println!("{}", json);
}
