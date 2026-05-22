# Integers

**Requirement:** We want an `add` function that adds two integers together.

## A library crate

So far we've used `cargo new` to create a binary crate — a program with a `main` function that runs. For this chapter we want a library: reusable code without a `main`. Cargo supports this with the `--lib` flag:

```
cargo new --lib integers
```

Open `integers/src/lib.rs`. Instead of `fn main()`, Cargo generates a different default:

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

This is helpful context, but we're going to write the chapter ourselves from scratch. Replace the file contents with just the test module — no implementation yet:

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

Run the tests:

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

`add` doesn't exist yet, so the compiler can't find it. The warning about `use super::*` is because there's nothing in the parent module to import yet — it'll go away once we add the function.

Notice the compiler's suggestion: `2.add(2)`. That hint is pointing at a trait (`Add`) from the standard library. Traits are a major topic for later — for now, ignore the suggestion and write the function ourselves.

## Make it pass

Add `add` above the test module:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run the tests:

```
running 1 test
test tests::add_two_integers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests integers

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Green. You'll notice two sections in the output: the unit tests, and `Doc-tests integers`. Doc-tests are Rust examples embedded in documentation comments — there are none yet, so it shows 0. We'll come back to that.

The type is `i32` — a 32-bit signed integer. Rust has a full family of integer types: `i8`, `i16`, `i32`, `i64`, `i128` (signed) and `u8`, `u16`, `u32`, `u64`, `u128` (unsigned). The number is the bit width; `u` means "unsigned" (non-negative only). `i32` is the default when Rust infers an integer type and has no reason to choose otherwise.

Unlike some languages, Rust will not silently coerce between integer types. If you have an `i32` and a function expects `i64`, you need to convert explicitly. The compiler will tell you.

Commit.

## Documenting our code

Rust has a built-in documentation tool: `cargo doc`. It generates HTML docs from your source code. The interesting part is that you can write examples in your documentation comments — and Rust will compile and run them as tests.

Documentation comments use `///` (three slashes, not two). Add one above `add`:

```rust
/// Adds two integers together.
///
/// # Examples
///
/// ```
/// assert_eq!(integers::add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Run the tests:

```
running 1 test
test tests::add_two_integers ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests integers

running 1 test
test src/lib.rs - add (line 5) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The `Doc-tests integers` section now shows 1 test passing — the example in the `///` comment. This is one of Rust's better ideas: documentation that goes stale breaks the build. If you change the function and forget to update the example, `cargo test` will catch it.

Run `cargo doc --open` to build the docs and open them in a browser:

```
 Documenting integers v0.1.0 (/path/to/integers)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
   Generated /path/to/integers/target/doc/integers/index.html
```

You'll see the `add` function with your description and the example rendered as a code block. This is exactly how the standard library docs are written — the same tool, the same format.

Commit.

## A note on overflow

What happens if you add two very large integers and the result doesn't fit in `i32`? In debug builds (which is what `cargo test` and `cargo run` use), Rust **panics** — the program crashes with an explicit error rather than silently wrapping around. In release builds (`cargo build --release`), it wraps by default.

You can also opt in to explicit behaviour using methods like `.checked_add()`, `.saturating_add()`, or `.wrapping_add()`. For now, just know that Rust doesn't silently discard the problem.

## Wrapping up

### Rust concepts introduced

- `cargo new --lib` — library crates vs binary crates
- `i32` — the default integer type; brief overview of the integer family
- No silent coercion between integer types
- `///` doc comments
- `cargo doc` — generating HTML documentation from source
- Doc-tests — examples in `///` comments that `cargo test` compiles and runs
- Overflow behaviour: panic in debug, wrap in release

### Testing concepts

- Doc-tests are tests — examples that go out of date will fail the build
- `cargo test` runs both unit tests and doc-tests

See [The TDD Cycle](../principles/tdd-cycle.md) for more on why each step matters.
