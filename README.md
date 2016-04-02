# markov_rs

A simple implementation of a Markov chain text generator in Rust.

## Usage

Requires [Rust](https://www.rust-lang.org/).

Build with:

    cargo build --release

Then provide input text via standard in and the maximum number of words to
generate as an argument:

    cat alice.txt | ./target/release/markov_rs 20
