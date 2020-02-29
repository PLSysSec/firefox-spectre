# wast

**A [Bytecode Alliance](https://bytecodealliance.org/) project**

A Rust parser for the WebAssembly Text format: [WAT][wat] and WAST

[![Documentation](https://docs.rs/wast/badge.svg)](https://docs.rs/wast)

[wat]: http://webassembly.github.io/spec/core/text/index.html

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
wast = "3.0"
```

The intent of this crate is to provide utilities, combinators, and built-in
types to parse anything that looks like a WebAssembly s-expression.

* Need to parse a `*.wat` file?
* Need to parse a `*.wast` file?
* Need to run test suite assertions from the official wasm test suite?
* Want to write an extension do the WebAssembly text format?

If you'd like to do any of the above this crate might be right for you! You may
also want to check out the `wat` crate which provides a much more stable
interface if all you'd like to do is convert `*.wat` to `*.wasm`.

## Cargo features

By default this crate enables and exports support necessary to parse `*.wat` and
`*.wast` files, or in other words entire wasm modules. If you're using this
crate, however, to parse simply an s-expression wasm-related format (like
`*.witx` or `*.wit` perhaps) then you can disable the default set of features to
only include a lexer, the parsing framework, and a few basic token-related
parsers.

```toml
[dependencies]
wast = { version = "3.0", default-features = false }
```

# License

This project is license under the Apache 2.0 license with the LLVM exception.
See [LICENSE] for more details.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
