Nice example of using polly with [rust](https://github.com/rust-lang/rust/pull/78566) using `-C llvm-args="--polly --polly-vectorizer=stripmine"`

Puffin
-----
[Puffin](https://github.com/EmbarkStudios/puffin) is a benchmarking tool with need a neat ui.

cargo-msrv
-----
[cargo-msrv](https://crates.io/crates/cargo-msrv) is a tool that finds the minimum supported rust version

cargo-valgrind 
-----
[cargo-valgrind](https://github.com/jfrimmel/cargo-valgrind) is a tool that runs valgrind and displays its output in a helpful manner.

Usage:
```bash 
cargo valgrind run
cargo valgrind test
cargo valgrind bench
```

cargo-clone
-----
[cargo-clone](https://github.com/JanLikar/cargo-clone) is a tool that fetch the source code of a crate.

cargo-show-asm
-----
[cargo-show-asm](https://github.com/pacak/cargo-show-asm) is a tool showing  the assembly LLVM-IR and MIR.

```bash
# here we are targeting lib in examplerust crate
cargo asm -p examplerust --lib   
# here "from_" is part of the function you are interested in intel syntax
cargo asm -p examplerust --lib from_ 
# get the full result
cargo asm -p isin --lib examplerust::source1::example_bench1
```

cargo-flamegraph
-----
[cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph) is a tool generating flamegraphs.

Usage:
```bash 
flamegraph -- /path/to/binary
```

cargo-pgo
-----
[cargo-flamegraph](https://github.com/Kobzol/cargo-pgo) is a tool for creating PGOs

needs: 
```
rustup component add llvm-tools-preview
```

PGO:
```bash
# build
cargo pgo instrument build
# collect
cargo pgo instrument bench //or: cargo pgo instrument test // or: cargo pgo instrument run
# optimize
cargo pgo optimize bench // or cargo pgo optimize test
```

BOLT:
```bash 
cargo pgo bolt build
```

BOLT + PGO
```bash
# Build PGO instrumented binary
$ cargo pgo build
# Run binary to gather PGO profiles
$ ./target/.../<binary>
# Build BOLT instrumented binary using PGO profiles
$ cargo pgo bolt build --with-pgo
# Run binary to gather BOLT profiles
$ ./target/.../<binary>-bolt-instrumented
# Optimize a PGO-optimized binary with BOLT
$ cargo pgo bolt optimize --with-pgo
```

Cargo Configuration
-----

[See](https://doc.rust-lang.org/cargo/reference/config.html) for more information
about compiler optimizations.

# Examples:

## Benches
```bash 
cargo bench
```

## Test
```bash 
cargo test
```
