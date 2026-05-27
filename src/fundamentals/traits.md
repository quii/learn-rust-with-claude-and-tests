# Traits

At the end of the last chapter, `Rectangle` and `Circle` both have an `area()` method. But they have no connection in the type system. You can't write a single function that accepts either one.

**Requirement:** We want a function that can describe the area of any shape.

## The problem

This chapter continues in the `structs` crate. Add a test for a `describe_area` function that takes a `Rectangle`:

```rust
#[test]
fn description_of_rectangle() {
    let r = Rectangle { width: 10.0, height: 10.0 };
    assert_eq!(describe_area(&r), "This shape has an area of 100");
}
```

Run `cargo test`:

```
error[E0425]: cannot find function `describe_area` in this scope
  --> src/lib.rs:53:20
   |
53 |         assert_eq!(describe_area(&r), "This shape has an area of 100");
   |                    ^^^^^^^^^^^^^ not found in this scope
```

Add the function:

```rust
pub fn describe_area(r: &Rectangle) -> String {
    format!("This shape has an area of {}", r.area())
}
```

A quick note on the format string: `{}` on a whole-number `f64` like `100.0` prints `100`, not `100.0`. That's why the test expects `"100"` rather than `"100.0"`. Rust's default float formatting drops trailing zeros and the decimal point when they're not needed.

Run `cargo test`:

```
running 4 tests
test tests::area_of_circle ... ok
test tests::area_of_rectangle ... ok
test tests::description_of_rectangle ... ok
test tests::perimeter_of_rectangle ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Commit.

## The wall

Now add a test for `Circle`:

```rust
#[test]
fn description_of_circle() {
    let c = Circle { radius: 10.0 };
    assert_eq!(describe_area(&c), "This shape has an area of 314.1592653589793");
}
```

Run `cargo test`:

```
error[E0308]: mismatched types
  --> src/lib.rs:63:34
   |
63 |         assert_eq!(describe_area(&c), "This shape has an area of 314.1592653589793");
   |                    ------------- ^^ expected `&Rectangle`, found `&Circle`
   |                    |
   |                    arguments to this function are incorrect
   |
   = note: expected reference `&Rectangle`
              found reference `&Circle`
```

`describe_area` only accepts `&Rectangle`. There's no type you can put in the signature that means "anything with an `area` method". This is the problem that traits solve.

## Defining a trait

A trait defines a contract — a named set of method signatures that a type must implement. Define one:

```rust
pub trait Shape {
    fn area(&self) -> f64;
}
```

This says: any type that wants to be a `Shape` must have an `area` method that takes `&self` and returns `f64`. Nothing more.

Adding this to your file and running `cargo test` won't change the error yet — `describe_area` still expects `&Rectangle`, so the compiler still reports the same mismatch:

```
error[E0308]: mismatched types
  --> src/lib.rs:68:34
   |
68 |         assert_eq!(describe_area(&c), "This shape has an area of 314.1592653589793");
   |                    ------------- ^^ expected `&Rectangle`, found `&Circle`
   |                    |
   |                    arguments to this function are incorrect
   |
   = note: expected reference `&Rectangle`
              found reference `&Circle`
note: function defined here
  --> src/lib.rs:33:8
   |
33 | pub fn describe_area(r: &Rectangle) -> String {
   |        ^^^^^^^^^^^^^ -------------
```

Defining a trait doesn't sign anyone up to it. The next step is implementing it for each type and updating `describe_area` to use the trait as its bound.

## Implementing the trait

`impl Shape for Rectangle` signs the contract for `Rectangle`. The syntax mirrors the `impl Rectangle` blocks you've already written — same keyword, different purpose.

First, update `describe_area` to accept any type that implements `Shape`:

```rust
pub fn describe_area(shape: &impl Shape) -> String {
    format!("This shape has an area of {}", shape.area())
}
```

`&impl Shape` means "a reference to some concrete type that implements `Shape`". The compiler resolves the actual type at compile time — there's no runtime overhead.

Run `cargo test`:

```
error[E0277]: the trait bound `Rectangle: Shape` is not satisfied
  --> src/lib.rs:61:34
   |
61 |         assert_eq!(describe_area(&r), "This shape has an area of 100");
   |                    ------------- ^^ unsatisfied trait bound
   |                    |
   |                    required by a bound introduced by this call
   |
help: the trait `Shape` is not implemented for `Rectangle`
  --> src/lib.rs:3:1
   |
 3 | pub struct Rectangle {
   | ^^^^^^^^^^^^^^^^^^^^
help: this trait has no implementations, consider adding one
  --> src/lib.rs:27:1
   |
27 | pub trait Shape {
   | ^^^^^^^^^^^^^^^
note: required by a bound in `describe_area`
  --> src/lib.rs:32:35
   |
32 | pub fn describe_area(shape: &impl Shape) -> String {
   |                                   ^^^^^ required by this bound in `describe_area`
```

Now the compiler knows exactly what it needs. Sign the contract for both types. Note that the `area` method that previously lived in `impl Rectangle` and `impl Circle` now lives here instead — remove those old methods when you add these blocks:

```rust
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}
```

Run `cargo test`:

```
running 5 tests
test tests::area_of_circle ... ok
test tests::area_of_rectangle ... ok
test tests::description_of_circle ... ok
test tests::description_of_rectangle ... ok
test tests::perimeter_of_rectangle ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Commit.

## Adding a Triangle

Add the struct and test first:

```rust
pub struct Triangle {
    pub base: f64,
    pub height: f64,
}
```

```rust
#[test]
fn description_of_triangle() {
    let t = Triangle { base: 5.0, height: 10.0 };
    assert_eq!(describe_area(&t), "This shape has an area of 25");
}
```

Run `cargo test`:

```
error[E0277]: the trait bound `Triangle: Shape` is not satisfied
  --> src/lib.rs:69:34
   |
69 |         assert_eq!(describe_area(&t), "This shape has an area of 25");
   |                    ------------- ^^ unsatisfied trait bound
   |                    |
   |                    required by a bound introduced by this call
   |
help: the trait `Shape` is not implemented for `Triangle`
  --> src/lib.rs:12:1
   |
12 | pub struct Triangle {
   | ^^^^^^^^^^^^^^^^^^^
help: the following other types implement trait `Shape`
  --> src/lib.rs:21:1
   |
21 | impl Shape for Rectangle {
   | ^^^^^^^^^^^^^^^^^^^^^^^^ `Rectangle`
...
27 | impl Shape for Circle {
   | ^^^^^^^^^^^^^^^^^^^^^ `Circle`
note: required by a bound in `describe_area`
  --> src/lib.rs:34:35
   |
34 | pub fn describe_area(shape: &impl Shape) -> String {
   |                                   ^^^^^ required by this bound in `describe_area`
```

The compiler even tells you which types already implement `Shape`. Implement it for `Triangle` (area = half base × height):

```rust
impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}
```

Run `cargo test`:

```
running 6 tests
test tests::area_of_circle ... ok
test tests::area_of_rectangle ... ok
test tests::description_of_circle ... ok
test tests::description_of_rectangle ... ok
test tests::description_of_triangle ... ok
test tests::perimeter_of_rectangle ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`describe_area` didn't need to change at all. That's the point — the trait is the stable contract. New shapes plug in without touching existing code.

Commit.

## Table tests

Three separate description tests are testing the same behaviour with different inputs. This is a good candidate for a table test — a list of cases run in a loop.

This is a restructuring of the tests, not a change to what they assert. The existing tests are already green; we're reorganising them. Since we're not changing behaviour, we should not need to change any assertions — the existing expected values stay exactly as they are.

Replace the three individual description tests with a single table test:

```rust
#[test]
fn description_of_shapes() {
    let shapes: Vec<(&str, &dyn Shape, &str)> = vec![
        ("rectangle", &Rectangle { width: 10.0, height: 10.0 }, "This shape has an area of 100"),
        ("circle", &Circle { radius: 10.0 }, "This shape has an area of 314.1592653589793"),
        ("triangle", &Triangle { base: 5.0, height: 10.0 }, "This shape has an area of 25"),
    ];

    for (name, shape, want) in shapes {
        assert_eq!(describe_area(shape), want, "failed for {}", name);
    }
}
```

Run `cargo test`:

```
error[E0277]: the size for values of type `dyn Shape` cannot be known at compilation time
  --> src/lib.rs:75:38
   |
75 |             assert_eq!(describe_area(shape), want, "failed for {}", name);
   |                        ------------- ^^^^^ doesn't have a size known at compile-time
   |                        |
   |                        required by a bound introduced by this call
   |
   = help: the trait `Sized` is not implemented for `dyn Shape`
note: required by an implicit `Sized` bound in `describe_area`
  --> src/lib.rs:40:30
   |
40 | pub fn describe_area(shape: &impl Shape) -> String {
   |                              ^^^^^^^^^^ required by the implicit `Sized` requirement on this type parameter in `describe_area`
help: consider relaxing the implicit `Sized` restriction
   |
40 | pub fn describe_area(shape: &impl Shape + ?Sized) -> String {
   |                                         ++++++++
```

The compiler is pointing at the root of the problem. `&impl Shape` works for a *single* call where the concrete type is known at compile time — it secretly requires `Sized`. A `Vec` must hold elements of a single, uniform type, but `Rectangle`, `Circle`, and `Triangle` are three different concrete types. They can't all live in the same `Vec<&impl Shape>`.

`&dyn Shape` is a *trait object* — a fat pointer that carries both a reference to the value and a pointer to a vtable of method implementations. The compiler doesn't need to know the concrete type at compile time; it resolves `area()` at runtime through the vtable. That's the cost: a small runtime indirection. The benefit: a heterogeneous collection of any type implementing `Shape`.

Update `describe_area`:

```rust
pub fn describe_area(shape: &dyn Shape) -> String {
    format!("This shape has an area of {}", shape.area())
}
```

Run `cargo test`:

```
running 4 tests
test tests::area_of_circle ... ok
test tests::area_of_rectangle ... ok
test tests::description_of_shapes ... ok
test tests::perimeter_of_rectangle ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

A few more things to unpack about the table test:

`Vec<...>` is Rust's growable array type. `vec![...]` is the macro that creates one.

Each element is a tuple of three values: a name (`&str`), a shape (`&dyn Shape`), and the expected output (`&str`). Tuples group values of different types — you access them by position in a `for` destructure: `(name, shape, want)`.

The third argument to `assert_eq!` is an optional failure message. Without it, a failing table test only shows you the mismatched values — not *which case* failed. With `"failed for {}", name` you get:

```
assertion `left == right` failed: failed for rectangle
  left: "This shape has an area of 100"
 right: "This shape has an area of 999"
```

Before committing, deliberately break one assertion — change an expected value to something wrong — and run `cargo test` to confirm the failure message is useful. A test you've never seen fail is a test you can't fully trust. Once you're satisfied, restore it and commit.

## Wrapping up

### Rust concepts introduced

- `trait` — a named contract: a set of method signatures a type must implement
- `impl Trait for Type` — explicitly signs the contract for a type; the compiler enforces it
- `&impl Shape` — a function parameter that accepts any type implementing `Shape`, resolved at compile time
- `&dyn Shape` — a trait object; accepts any type implementing `Shape`, resolved at runtime; required for heterogeneous collections
- `Vec<T>` and `vec![]` — growable array and its creation macro
- Tuples — grouping values of different types; destructured in `for` loops
- `assert_eq!` failure message — third argument formats a message shown when the assertion fails

### Testing concepts

- Table tests reduce repetition when you're testing the same behaviour with different inputs
- When restructuring tests, don't change the assertions — the existing expected values are your safety net
- After restructuring, break one assertion deliberately to confirm the test can still fail and the failure message is useful

### A note on Go

If you're coming from Go, Rust's explicit `impl Trait for Type` may feel verbose compared to Go's implicit interface satisfaction. The tradeoff is deliberate: in Rust you can always see exactly which traits a type implements by reading the file. Neither approach is objectively better — they reflect different priorities.

### Coming up

`&impl Shape` and `&dyn Shape` are two ways to use traits in function signatures. There's a third — generics (`fn foo<T: Shape>(s: &T)`) — which gives you more flexibility at the cost of more syntax. Standard library traits like `Display`, `Debug`, and `Iterator` are also worth knowing; they unlock a lot of Rust's ergonomics. Both are coming in later chapters.

See [Test Behaviour, Not Implementation](../principles/test-behaviour-not-implementation.md) and [The TDD Cycle](../principles/tdd-cycle.md).

### Further reading

- [Traits: defining shared behaviour](https://doc.rust-lang.org/book/ch10-02-traits.html) — default implementations, trait bounds, and the full `impl Trait` vs generic syntax comparison
- [Trait objects (`dyn`)](https://doc.rust-lang.org/book/ch18-02-trait-objects.html) — how vtables work, when to prefer `&dyn Trait` over generics, and the performance tradeoffs
- [Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html) — how `Vec<T>` manages memory, and the full range of operations available
