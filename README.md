# Syn-Zeug
A modern toolbox for synthetic biology

## Where Is Everything?
- `syn-zeug` is the core Rust library where biotools are implemented
- `web` is the Svelte SPA that makes up the web interface for syn-zeug
- `biobox` is a Rust shim that exposes a nice WASM interface to syn-zeug

## Currently Implemented Tools
### Basic
- Sequence Validation
- Sequence Length
- Count Sequence Elements (Bases / Residues)
- Reverse Complement
- Sequence Conversion
  - DNA -> RNA
