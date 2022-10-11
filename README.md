# Syn-Zeug

A modern toolbox for synthetic biology

## How Can I Use It?

Though you could download the code and build SZ yourself, there is a pre-built and automatically updated version of the web UI available at <https://sheffield-igem.github.io/syn-zeug/>.

## Where Is Everything?

- `syn-zeug` is the core Rust library where biotools are implemented
- `web` is the Svelte SPA that makes up the web interface for syn-zeug
- `biobox` is a Rust shim that exposes a nice WASM interface to syn-zeug

## Currently Implemented Tools

### In The Web UI

- Sequence Validation
- Sequence Length
- Reverse Sequence
- Count Sequence Elements (Bases / Residues)
- Reverse Complement
- Convert Case (Upper / Lower)
- Sequence Conversion
  - DNA -> RNA
  - RNA -> DNA
  - DNA -> Protein
  - RNA -> Protein
- GC Content
- Find Open Reading Frames

### In The Rust Library

- Extract Subsequences
- Hamming Distance
- Levenshtein Distance
