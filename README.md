# Legal Entity Identifier Rust library

[![Crates.io](https://img.shields.io/crates/v/leim.svg)](https://crates.io/crates/leim)
[![Documentation](https://docs.rs/leim/badge.svg)](https://docs.rs/leim/)

`leim` is a Rust library for working with Legal Entity Identifiers (LEIs) as defined
in [ISO 17442-1:2020](https://www.iso.org/standard/78829.html).

## Example

```rust
fn main() {
    use leim as lei;
    assert!(lei::LEI::try_from("2594007XIACKNMUAW223").is_ok());
    assert_eq!(
        lei::LEI::try_from("2594007XIACKNMUAW222"),
        Err(lei::Error::InvalidChecksum)
    );
}
```

## Usage

Add `leim` to your `Cargo.toml`:

```sh
cargo add leim
```

## Alternative crates

[`lei`](https://crates.io/crates/lei) is another crate for working with LEIs.

## Authors

This crate is developed and maintained by [21 Analytics](https://21analytics.ch).

## License

This project is licensed under the MIT license.
