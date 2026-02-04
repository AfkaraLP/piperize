# Piperize

A Rust crate inspired by elixir's pipe function declarations. This gets rid of the boilerplate of writing a new trait if you want to create a dot method on some type

## Usage:

```rust
use piperize::piperize;

#[piperize]
fn add_tuple_vals(a: (i32, i32)) -> i32 {
  a.0 + a.1
}

fn main() {
  assert_eq!(42, (67, -25).add_tuple_vals());
}
```
## Using multiple arguments:

```rust
use piperize::piperize;

#[piperize]
fn my_add(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    let foo = 21.my_add(21);
    assert_eq!(foo, 42);
}
```
