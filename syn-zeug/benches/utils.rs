use std::fs;

pub fn load_bench_data(f: &str) -> Vec<u8> {
    fs::read_to_string(["benches/data/", f].concat())
        .unwrap()
        .trim()
        .as_bytes()
        .to_vec()
}
