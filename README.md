# Chase [![Build Status](https://travis-ci.org/lloydmeta/chase-rs.svg?branch=master)](https://travis-ci.org/lloydmeta/chase-rs) [![Crates.io](https://img.shields.io/crates/v/chase.svg)](https://crates.io/crates/chase) [![Chase](https://docs.rs/chase/badge.svg)](https://docs.rs/chase)

An implementation of async and sync file-following in Rust for people who care about line numbers.

### Goals

- Provide line numbers with each line yielded
- Ability to exit the watch loop programmatically
- Deals with file rotations automatically
- Cross-platform async 
- Configurable (which line to start on, delays and retries)
- Easy to use synchronously
- Easy to use asynchronously
  - Can receive data from a standard lib a [Channel](https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html)
  - Can receive data from a `Stream` from the [Futures library](https://github.com/alexcrichton/futures-rs)
  
### Usage

You can use this tool as a lib and as a binary:

#### As lib

Some features (e.g. receiving as a `Stream`, and Serde derive for lib-provided structs) are feature-gated, so
keep that in mind when adding as a dependency (refer to `Cargo.toml` for list of features)

#### As a binary

`cargo install chase --features=binary`

```shell
Chases a file through thick and thin.

USAGE:
    chase [OPTIONS] <f>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -L, --line <l>    The line you want to start chasing your file from [default: 0]

ARGS:
    <f>    The file you want to chase
```
  
### Caveats

Windows not yet supported: need to figure out what inodes map to

### Credit

Very much inspired by [logwatcher](https://github.com/aravindavk/logwatcher/)