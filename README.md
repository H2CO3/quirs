# Qui-RS

[![Qui-RS on crates.io](https://img.shields.io/crates/v/quirs.svg)](https://crates.io/crates/quirs)
[![Qui-RS on docs.rs](https://docs.rs/quirs/badge.svg)](https://docs.rs/quirs)
[![Qui-RS Download](https://img.shields.io/crates/d/quirs.svg)](https://crates.io/crates/quirs)
[![Qui-RS License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/H2CO3/quirs/blob/master/LICENSE.txt)
[![Twitter](https://img.shields.io/badge/twitter-@H2CO3_iOS-blue.svg?style=flat&colorB=64A5DE&label=Twitter)](http://twitter.com/H2CO3_iOS)

[![Lines of Code](https://tokei.rs/b1/github/H2CO3/quirs)](https://github.com/Aaronepower/tokei)
[![goto counter](https://img.shields.io/github/search/H2CO3/quirs/goto.svg)](https://github.com/H2CO3/quirs/search?q=goto)
[![fuck counter](https://img.shields.io/github/search/H2CO3/quirs/fuck.svg)](https://github.com/H2CO3/quirs/search?q=fuck)

A Rust wrapper around the QR decoder library [`quirc`](https://github.com/dlbeer/quirc/).

## Building

You'll need a working C toolchain (C compiler, linker, `make`, `pkg-config`)
in order to be able to build the bundled `quirc` library.

You will also need a Rust toolchain of version 1.24 or higher.

On Debian-based GNU/Linux distros, the following may work for installing GCC:

```shell
sudo apt install build-essential gcc make pkg-config
```

On macOS, you can just install the Xcode Command-Line Tools in order to have
a `clang`-based C toolchain.

Once you have the C and Rust toolchains, just run the following command from
the repo root:

```shell
git submodule update --init --recursive
cargo build
```
