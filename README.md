# RustVerbalExpressions

[![Travis](https://img.shields.io/travis/woxtu/RustVerbalExpressions.svg?style=flat-square)](https://travis-ci.org/woxtu/RustVerbalExpressions)
[![Crates.io](https://img.shields.io/crates/v/verbal_expressions.svg?style=flat-square)](https://crates.io/crates/verbal_expressions)

Verbal Expressions implementation for Rust. See [Verbal Expressions](http://verbalexpressions.github.io/) for detail.

## Install

Add the following to your `Cargo.toml`:

```toml
[dependencies]
verbal_expressions = "0.1.0"
```

## Example

```rust
extern crate verbal_expressions;
use verbal_expressions::Verex;

fn main() {
  let v = Verex::new()
    .start_of_line()
    .then("http")
    .maybe("s")
    .then("://")
    .maybe("www.")
    .anything_but_not(" ")
    .end_of_line();

  let url = "https://www.google.com";

  assert!(v.is_match(url));
}
```

## License
Copyright (c) 2014 woxtu

Licensed under the MIT license.
