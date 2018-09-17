# Blight Engine

[![Build status](https://ci.appveyor.com/api/projects/status/i4h48nhyq6f5i8f4?svg=true)](https://ci.appveyor.com/project/snorrwe/blight-engine)

## Requirements

- Rust nightly
- Cargo

## Building

```
cargo build            # Build the library
cargo build --examples # Build examples
cargo build --release  # Build in Release mode
```

## Running the tests

```
cargo test
```

## Distribution

__Blight Engine__ depends on __SDL2__. When distributing make sure to include the provided `SDL2.dll` with your executable!
