# Advent of Code

Solutions in Rust. Each day lives in `<year>/day/<n>/rust/<name>` with its puzzle input next to it in `<year>/day/<n>/inputs.txt`.

## Run

```sh
cd 2021/day/1/rust/sonar_sweep
cargo run --release
```

CI runs `cargo fmt`, `clippy` and `cargo test` for every crate. `nix develop` gives a shell with the toolchain.
