# Structs and Methods

**Requirement:** We want some geometry code to calculate the perimeter and area of shapes.

## Perimeter

Create a new library crate:

```
cargo new --lib structs
```

Write this test in `structs/src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perimeter_of_rectangle() {
        let got = perimeter(10.0, 10.0);
        assert_eq!(got, 40.0);
    }
}
```

Run `cargo test`:

```
error[E0425]: cannot find function `perimeter` in this scope
 --> src/lib.rs:7:19
  |
7 |         let got = perimeter(10.0, 10.0);
  |                   ^^^^^^^^^ not found in this scope

warning: unused import: `super::*`
 --> src/lib.rs:3:9
  |
3 |     use super::*;
  |         ^^^^^^^^
```

Add the function above the test module:

```rust
pub fn perimeter(width: f64, height: f64) -> f64 {
    2.0 * (width + height)
}
```

`f64` is Rust's 64-bit floating point type — the right choice for geometry. There's also `f32` but `f64` is the default and you'd need a specific reason to prefer the smaller type.

Run `cargo test`:

```
running 1 test
test tests::perimeter_of_rectangle ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Commit.

## Area

You have everything you need. Write a test for `area(width, height)` returning width × height, make it pass, and commit.

## Refactor

Look at the two functions:

```rust
pub fn perimeter(width: f64, height: f64) -> f64 { ... }
pub fn area(width: f64, height: f64) -> f64 { ... }
```

There's a problem here. Nothing stops a caller from passing the dimensions of a triangle and getting a wrong answer. The types don't encode the intent — two bare `f64` values could be anything.

We can fix this by defining our own type called `Rectangle` that makes the intent explicit. A `struct` is a named type with fields:

```rust
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
```

`pub` on the struct makes it visible outside this module. `pub` on each field is also required — without it, code outside the module couldn't read `r.width` or `r.height`.

Now update the tests to use `Rectangle` instead of bare floats:

```rust
#[test]
fn perimeter_of_rectangle() {
    let r = Rectangle { width: 10.0, height: 10.0 };
    let got = perimeter(r);
    assert_eq!(got, 40.0);
}

#[test]
fn area_of_rectangle() {
    let r = Rectangle { width: 10.0, height: 10.0 };
    let got = area(r);
    assert_eq!(got, 100.0);
}
```

`Rectangle { width: 10.0, height: 10.0 }` is a struct literal — you name each field. Run `cargo test`:

```
error[E0422]: cannot find struct, variant or union type `Rectangle` in this scope
15 |         let r = Rectangle { width: 10.0, height: 10.0 };

error[E0422]: cannot find struct, variant or union type `Rectangle` in this scope
22 |         let r = Rectangle { width: 10.0, height: 10.0 };

error[E0061]: this function takes 2 arguments but 1 argument was supplied
16 |         let got = perimeter(r);

error[E0061]: this function takes 2 arguments but 1 argument was supplied
23 |         let got = area(r);
```

The compiler tells you exactly what's broken. Add the struct and update both functions:

```rust
pub fn perimeter(r: Rectangle) -> f64 {
    2.0 * (r.width + r.height)
}

pub fn area(r: Rectangle) -> f64 {
    r.width * r.height
}
```

`r.width` and `r.height` are field accesses — the `.` operator reaches into the struct.

Run `cargo test`:

```
running 2 tests
test tests::area_of_rectangle ... ok
test tests::perimeter_of_rectangle ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Commit.

## Circle

Add a `Circle` struct and a test for its area:

```rust
pub struct Circle {
    pub radius: f64,
}
```

```rust
#[test]
fn area_of_circle() {
    let c = Circle { radius: 10.0 };
    let got = area(c);
    assert_eq!(got, 314.1592653589793);
}
```

Now try to write `area` for `Circle` as a free function, the same way you did for `Rectangle`:

```rust
pub fn area(c: Circle) -> f64 {
    0.0
}
```

Run `cargo test`:

```
error[E0428]: the name `area` is defined multiple times
  --> src/lib.rs:18:1
   |
14 | pub fn area(r: Rectangle) -> f64 {
   | -------------------------------- previous definition of the value `area` here
...
18 | pub fn area(c: Circle) -> f64 {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `area` redefined here
   |
   = note: `area` must be defined only once in the value namespace of this module
```

Rust won't allow two free functions with the same name, even with different parameter types. Unlike some languages, there's no function overloading.

This is the problem that *methods* solve.

## Methods

Instead of a free function `area` that takes a shape, each shape gets its own `area` method — `rectangle.area()` and `circle.area()`. They live on different types so there's no collision.

Start with the test. Update the circle test to express the API you want:

```rust
#[test]
fn area_of_circle() {
    let c = Circle { radius: 10.0 };
    assert_eq!(c.area(), 314.1592653589793);
}
```

Also delete the `area(c: Circle)` free function — you're replacing it. Run `cargo test`:

```
error[E0599]: no method named `area` found for struct `Circle` in the current scope
   |
 7 | pub struct Circle {
   | ----------------- method `area` not found for this struct
...
   |         assert_eq!(c.area(), 314.1592653589793);
   |                      ^^^^ method not found in `Circle`
```

The compiler is telling you exactly what's missing. Now add it.

A method is defined in an `impl` block:

```rust
impl Circle {
    pub fn area(&self) -> f64 {
        todo!()
    }
}
```

`impl Circle` opens a block where you define methods that belong to `Circle`. The first parameter `&self` is how a method refers to the value it's called on — `self` is the circle, `&` means it's borrowed rather than owned. Inside the method you access fields as `self.radius`.

The disciplined move is to get back to green as quickly as possible — fix `Circle` first, without touching the `Rectangle` code that's still working.

For the circle area formula you'll need π. It lives in the standard library:

```rust
use std::f64::consts::PI;
```

Put that at the top of the file. The area of a circle is π × r²:

```rust
impl Circle {
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}
```

Run `cargo test`:

```
running 3 tests
test tests::area_of_circle ... ok
test tests::area_of_rectangle ... ok
test tests::perimeter_of_rectangle ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Green. Now do the same for `Rectangle`. But follow the same discipline — tests first. Update both rectangle tests to call `r.perimeter()` and `r.area()` instead of the free functions. Run `cargo test` and see them fail, then update the production code to match.

Once you're green again, your final `lib.rs` should look like this:

```rust
use std::f64::consts::PI;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Circle {
    pub radius: f64,
}

impl Rectangle {
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Circle {
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perimeter_of_rectangle() {
        let r = Rectangle { width: 10.0, height: 10.0 };
        assert_eq!(r.perimeter(), 40.0);
    }

    #[test]
    fn area_of_rectangle() {
        let r = Rectangle { width: 10.0, height: 10.0 };
        assert_eq!(r.area(), 100.0);
    }

    #[test]
    fn area_of_circle() {
        let c = Circle { radius: 10.0 };
        assert_eq!(c.area(), 314.1592653589793);
    }
}
```

Commit.

## Wrapping up

### Rust concepts introduced

- `f64` — 64-bit floating point; the default for decimals
- `struct` — a named type with fields; groups related data and encodes intent in the type system
- `pub` on struct and fields — required for visibility outside the module
- Struct literal syntax — `Rectangle { width: 10.0, height: 10.0 }`
- Field access with `.` — `r.width`, `r.height`
- `impl` — attaches methods to a type
- `&self` — borrows the receiver; the method can read the struct's fields without taking ownership
- `use std::f64::consts::PI` — importing a constant from the standard library

### Testing concepts

- Write the test before the type exists — the compiler error tells you exactly what to build
- Fix one thing at a time — when the build is broken, restore green before adding the next requirement

### Coming up

`Rectangle` and `Circle` both have an `area` method, but they have no connection in the type system. You can't write a function that accepts either one. That's what traits are for — the next chapter.

See [The TDD Cycle](../principles/tdd-cycle.md) and [Separating Concerns](../principles/separating-concerns.md).
