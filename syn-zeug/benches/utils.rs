use std::fs;

pub fn load_bench_data(f: impl AsRef<str>) -> Vec<u8> {
    fs::read_to_string(["benches/data/", f.as_ref()].concat())
        .unwrap()
        .trim()
        .as_bytes()
        .to_vec()
}
