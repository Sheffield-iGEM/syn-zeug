## Code Checks

```bash
# Basic checks
cargo test && cargo fmt && cargo clippy
# Test coverage
cargo tarpaulin --lib
# Benchmarking
cargo bench
# Profiling
cargo bench --bench criterion -- --profile-time 10
```
