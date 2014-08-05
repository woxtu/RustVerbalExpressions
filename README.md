# RustVerbalExpressions

Verbal Expressions implementation for Rust. See [Verbal Expressions](http://verbalexpressions.github.io/) for detail.

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
