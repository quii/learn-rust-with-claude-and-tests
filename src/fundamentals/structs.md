# Structs

**Requirement:** We want to calculate the perimeter and area of rectangles.

## Write the test first

Create a new library crate:

```
cargo new --lib structs
```

Replace `structs/src/lib.rs` with just a test. We're going to write what we *wish* existed — a `Rectangle` type and a `perimeter` function — before either of them does:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perimeter_of_rectangle() {
        let r = Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(perimeter(r), 30.0);
    }
}
```

Run `cargo test`:

```
error[E0422]: cannot find struct, variant or union type `Rectangle` in this scope
 --> src/lib.rs:7:17
  |
7 |         let r = Rectangle { width: 10.0, height: 5.0 };
  |                 ^^^^^^^^^ not found in this scope

warning: unused import: `super::*`
 --> src/lib.rs:3:9
  |
3 |     use super::*;
  |         ^^^^^^^^

error[E0425]: cannot find function `perimeter` in this scope
 --> src/lib.rs:8:20
  |
8 |         assert_eq!(perimeter(r), 30.0);
  |                    ^^^^^^^^^ not found in this scope
```

Two errors. Fix them one at a time. Define the struct first:

```rust
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
```

`struct` defines a named type with fields. The `pub` on the struct makes it visible outside the module — as does `pub` on each field. Without `pub` on the fields, code outside this module couldn't read `r.width` or `r.height`.

`f64` is a 64-bit floating point number — Rust's default for decimals. There's also `f32` but unless you have a specific reason to prefer it, `f64` is what you want.

Run `cargo test`:

```
error[E0425]: cannot find function `perimeter` in this scope
  --> src/lib.rs:13:20
   |
13 |         assert_eq!(perimeter(r), 30.0);
   |                    ^^^^^^^^^ not found in this scope
```

One error left. Add the function:

```rust
pub fn perimeter(r: Rectangle) -> f64 {
    2.0 * (r.width + r.height)
}
```

Run `cargo test`:

```
running 1 test
test tests::perimeter_of_rectangle ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Green. `r.width` and `r.height` are field accesses — the `.` operator reaches into the struct. The formula for perimeter is `2 * (width + height)`.

Commit.

## Methods

The function works, but `perimeter(r)` doesn't feel like it belongs to `Rectangle`. In Rust you attach behaviour to a type using `impl`. Update the test first:

```rust
#[test]
fn perimeter_of_rectangle() {
    let r = Rectangle { width: 10.0, height: 5.0 };
    assert_eq!(r.perimeter(), 30.0);
}
```

Run `cargo test`:

```
error[E0599]: no method named `perimeter` found for struct `Rectangle` in the current scope
  --> src/lib.rs:8:22
   |
 8 |         assert_eq!(r.perimeter(), 30.0);
   |                      ^^^^^^^^^ method not found in `Rectangle`
...
12 | pub struct Rectangle {
   | -------------------- method `perimeter` not found for this struct
```

Replace the standalone function with an `impl` block:

```rust
impl Rectangle {
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
```

Run `cargo test`:

```
running 1 test
test tests::perimeter_of_rectangle ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`impl Rectangle` opens a block where you define methods that belong to `Rectangle`. The first parameter `&self` is how a method borrows the value it's called on — `self` is the rectangle, `&` means we're borrowing it rather than taking ownership. Inside the method, `self.width` and `self.height` replace `r.width` and `r.height`.

This matters: the standalone function took `r: Rectangle` — it *owned* the rectangle, consuming it. After calling `perimeter(r)`, `r` is gone. The method takes `&self` — it only borrows, so `r` is still usable after `r.perimeter()`. For a simple calculation that's the right choice.

Commit.

## Area

The pattern is established. Add the test:

```rust
#[test]
fn area_of_rectangle() {
    let r = Rectangle { width: 10.0, height: 5.0 };
    assert_eq!(r.area(), 50.0);
}
```

Run `cargo test`:

```
error[E0599]: no method named `area` found for struct `Rectangle` in the current scope
  --> src/lib.rs:14:22
   |
14 |         assert_eq!(r.area(), 50.0);
   |                      ^^^^ method not found in `Rectangle`
...
18 | pub struct Rectangle {
   | -------------------- method `area` not found for this struct
```

Add `area` to the `impl` block alongside `perimeter`:

```rust
impl Rectangle {
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}
```

Run `cargo test`:

```
running 2 tests
test tests::area_of_rectangle ... ok
test tests::perimeter_of_rectangle ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Both green. The `impl` block grows with the type — all the behaviour in one place.

Commit.

## Wrapping up

### Rust concepts introduced

- `struct` — a named type with fields; groups related data together
- `pub` on fields — required to access fields from outside the module
- `f64` — 64-bit floating point; the default for decimals
- Field access with `.` — `r.width`, `r.height`
- `impl` — attaches methods to a type
- `&self` — borrows the receiver; the method can read the struct's fields without consuming it
- Ownership contrast: `r: Rectangle` takes ownership, `&self` borrows

### Testing concepts

- Writing the test before the type exists forces you to think about the API first
- Fixing compiler errors one at a time keeps the feedback loop tight

See [The TDD Cycle](../principles/tdd-cycle.md) and [Separating Concerns](../principles/separating-concerns.md).
