# rust

## Development environment

On MacOS where terminal, XCode, and homebrew already installed:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install libraries (crates?)

```
source "$HOME/.cargo/env"
brew install gsl
cargo install rust-bio-tools 
```

## Run program

```
# cargo new for_fun --bin # hmm
cargo run

    Finished dev [unoptimized + debuginfo] target(s) in 0.14s
     Running `target/debug/hello_world`
[65, 67, 71, 84, 65, 67, 71, 84]
10000100001000011000010000100001
[67, 71, 67, 71, 67, 71, 71, 71]
01000010010000100100001000100010
```

## Pick a profiler

options: rustup, cargo flamegraph, heaptrack, valgrind, massif, ...