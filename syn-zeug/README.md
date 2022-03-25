# Syn-Zeug

A modern toolbox for synthetic biology

## Code Checks

``` bash
# Basic checks
cargo test && cargo fmt && cargo clippy
# Test coverage
cargo tarpaulin --lib
# Benchmarking
cargo bench
# Profiling
cargo bench --bench criterion -- --profile-time 10
```

## Currently Implemented Tools
### Basic
- Sequence Validation
- Sequence Length
- Count Sequence Elements (Bases / Residues)
- Reverse Complement
- Sequence Conversion
  - DNA -> RNA
