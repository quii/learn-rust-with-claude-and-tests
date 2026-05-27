# Integers

**Requirement:** We want an `add` function that adds two integers together.

## A library crate

So far we've used `cargo new` to create a binary crate — a program with a `main` function. For this chapter we want a *library*: reusable code without a `main`. The flag is `--lib`:

```
cargo new --lib integers
```

Open `integers/src/lib.rs`. You'll notice Cargo has scaffolded something more opinionated than `main.rs` — it generates an `add` stub and a passing test out of the box:

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

Convenient, but we're going to follow the journey ourselves. Replace the file contents with just the test — no implementation yet:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_integers() {
        assert_eq!(add(2, 2), 4);
    }
}
```

## Write the test first

Run `cargo test`:

```
warning: unused import: `super::*`
 --> src/lib.rs:3:9
  |
3 |     use super::*;
  |         ^^^^^^^^

error[E0425]: cannot find function `add` in this scope
 --> src/lib.rs:7:20
  |
7 |         assert_eq!(add(2, 2), 4);
  |                    ^^^ not found in this scope
  |
help: use the `.` operator to call the method `Add::add` on `{integer}`
  |
7 -         assert_eq!(add(2, 2), 4);
7 +         assert_eq!(2.add(2), 4);
```

`add` doesn't exist yet — exactly what we want. The warning about `use super::*` will disappear once there's something in the parent module to import.

Notice the compiler's suggestion: `2.add(2)`. That's pointing at the `Add` trait from the standard library. Traits are a major topic for later — ignore the hint for now and write the function ourselves.

## Make it pass

Add `add` above the test module:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run `cargo test`:

```
running 1 test
test tests::add_two_integers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests integers

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Green. Two things to note here.

First, the type is `i32` — a 32-bit signed integer. Rust has a full family: `i8`, `i16`, `i32`, `i64`, `i128` (signed) and `u8`, `u16`, `u32`, `u64`, `u128` (unsigned). `i32` is the default when Rust infers an integer type and has no reason to prefer otherwise.

Unlike some languages, Rust will not silently coerce between integer types. Pass an `i64` where `i32` is expected and the compiler rejects it — you have to convert explicitly. This is deliberate, and the compiler will tell you exactly what to do.

Second, notice every parameter has its own type annotation: `a: i32, b: i32`. There's no shorthand for consecutive parameters of the same type — Rust is explicit everywhere.

Commit.

## Documenting our code

Rust has a built-in documentation tool: `cargo doc`. It generates HTML docs from your source — the same tool used for the standard library. The interesting part is that examples in your doc comments are compiled and run as tests.

Documentation comments use `///` (three slashes). Add one above `add`:

```rust
/// Adds two integers together.
///
/// # Examples
///
/// ```
/// assert_eq!(integers::add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
```

Run `cargo test` and look at the output:

```
running 1 test
test tests::add_two_integers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests integers

running 1 test
test src/lib.rs - add (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

all doctests ran in 0.49s; merged doctests compilation took 0.24s
```

`Doc-tests integers` now shows 1 test passing — the example from the `///` comment. This is one of Rust's better ideas: documentation that goes stale breaks the build. Change `add` and forget to update the example, and `cargo test` will catch it.

Run `cargo doc --open` to build and view the docs:

```
 Documenting integers v0.1.0 (/path/to/integers)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
   Generated /path/to/integers/target/doc/integers/index.html
```

You'll see the description and example rendered exactly like the standard library docs — because it's the same tool.

Commit.

## A note on overflow

What happens when the result of an addition doesn't fit in `i32`? In debug builds — which is what `cargo test` and `cargo run` use — Rust **panics**. The program crashes with an explicit error rather than silently wrapping around. In release builds (`cargo build --release`) it wraps.

You can also opt in to explicit behaviour: `.checked_add()` returns `None` on overflow, `.saturating_add()` clamps to the maximum value, `.wrapping_add()` always wraps. For now, just know that Rust doesn't silently discard the problem.

## Wrapping up

### Rust concepts introduced

- `cargo new --lib` — library crates vs binary crates
- `i32` and the integer type family (`i8`–`i128`, `u8`–`u128`)
- No silent coercion between integer types
- `///` doc comments
- `cargo doc` — generating HTML documentation from source
- Doc-tests — examples in `///` comments that `cargo test` compiles and runs
- Overflow behaviour: panic in debug builds, wrap in release

### Testing concepts

- Doc-tests are tests — examples that go out of date will fail the build
- `cargo test` runs both unit tests and doc-tests automatically

See [The TDD Cycle](../principles/tdd-cycle.md) for more on why each step matters.

### Further reading

- [Integer types](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types) — the full family of signed and unsigned types, when to use each
- [Integer overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) — why debug and release builds behave differently, and what to do when you care
