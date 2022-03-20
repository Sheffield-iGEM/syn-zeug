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

## Goals

- Automated primer design
- Automated codon optimization
- Assisted bio-brick assembly / catalog exploration

## Tasks

### Basic
- [ ] Base Counting
- [ ] GC Content
- [ ] Transcription
- [ ] Reverse Transcription
- [ ] Translation
- [ ] Reverse Sequence
- [ ] Sequence Length
- [ ] Complement
- [ ] Hamming Distance
- [ ] General primer annealing temperature calculator 

### Miscellaneous

- [ ] Split up things into sensible modules
- [ ] Add code for computing GC-content (IUPAC compatible)
- [ ] Add an `rna()` constructor + tests + conversion / complementing code
- [ ] Fill in `todo!()`s
