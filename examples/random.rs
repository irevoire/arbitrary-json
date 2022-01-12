use arbitrary_json::ArbitraryValue;
use rand::Rng;

fn main() {
    let mut data = [0; 1000];
    rand::thread_rng().fill(&mut data[..]);

    let mut unstructured = arbitrary::Unstructured::new(&data);

    for _ in 0..100 {
        let json: ArbitraryValue = unstructured.arbitrary().unwrap();

        dbg!(json);
    }
}
