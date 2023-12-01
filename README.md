### Workshop solana program example

Some adoptation of https://github.com/solana-labs/example-helloworld

### To start

If you want to add missing code yourself following instructions from the talk, checkout start:
```shell
git checkout start
```

Master branch contains complete version.

### Build and test for program compiled natively
```shell
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```shelll
$ cargo build-bpf
$ cargo test-bpf
```
