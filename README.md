# sz_rust_bench

To run the benchmarks, you will need to install the cargo-fuzz command line helper for cargo. You can do
that by running the following command:
```bash
cargo install cargo-fuzz --locked
```

## Fuzzing

To list all the available fuzzing targets, you can run:
```bash
cargo fuzz list
```

To run a fuzz test, you can run:
```bash
cargo fuzz run <fuzz-target>
```

## Development

To add a fuzzing target, run:
```bash
# For memchr
cargo fuzz add memchr_<function_name>
# For StringZilla
cargo fuzz add sz_<function_name>
```
