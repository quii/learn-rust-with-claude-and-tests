# Strings and Iteration

**Requirement:** We want a `repeat` function that takes a string and a count, and returns the string repeated that many times. For example, `repeat("na", 4)` returns `"nananana"`.

## Write the test first

Create a new library crate:

```
cargo new --lib strings-and-iteration
```

Replace the contents of `strings-and-iteration/src/lib.rs` with just the test:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_a_string() {
        assert_eq!(repeat("na", 4), "nananana");
    }
}
```

Run `cargo test`:

```
error[E0425]: cannot find function `repeat` in this scope
 --> src/lib.rs:7:20
  |
7 |         assert_eq!(repeat("na", 4), "nananana");
  |                    ^^^^^^ not found in this scope
  |
help: consider importing one of these functions
  |
3 +     use std::array::repeat;
  |
3 +     use std::io::repeat;
  |
3 +     use std::iter::repeat;
  |
3 +     use core::array::repeat;
  |
  = and 1 other candidate

warning: unused import: `super::*`
 --> src/lib.rs:3:9
  |
3 |     use super::*;
  |         ^^^^^^^^
```

The compiler can't find `repeat` — and it's helpfully suggesting four alternatives from the standard library. Worth noting: `std::iter::repeat` is a real thing — an infinite iterator that repeats a value forever. Useful, but not what we want. We're building our own.

Ignore all four suggestions.

## Make it pass

Add `repeat` above the test module:

```rust
pub fn repeat(s: &str, n: usize) -> String {
    let mut result = String::new();
    for _ in 0..n {
        result.push_str(s);
    }
    result
}
```

Run `cargo test`:

```
running 1 test
test tests::repeat_a_string ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Green. There's a lot happening in a few lines — let's unpack it.

`String::new()` creates an empty, owned, heap-allocated string. `let mut result` — the `mut` is new. In Rust, variables are **immutable by default**. Without `mut`, you can't change `result` after it's created. Try removing it and the compiler will tell you:

```
error[E0596]: cannot borrow `result` as mutable, as it is not declared as mutable
```

`for _ in 0..n` — `0..n` is a **range**, producing the values 0, 1, 2... up to but not including `n`. The `_` means "I don't need the loop variable" — we just want to run the body `n` times.

`push_str` appends a `&str` to a `String` in place.

The last line — `result` with no semicolon — is the implicit return. We've seen this pattern before.

`n: usize` — why `usize` and not `i32`? Because `usize` is Rust's type for counts and indices: things that can't be negative. It's also what ranges produce. Using `i32` would work but feel off, and you'd hit type mismatches in certain contexts.

Commit.

## String vs &str

We've been using `&str` and `String` since Hello World without a proper explanation. Now that we're building one, it's time.

`&str` is a **string slice** — a reference to some string data that lives somewhere else. String literals like `"na"` are `&str`; the data is baked into the binary. You can read a `&str` but you can't grow or modify it.

`String` is an **owned string** — heap-allocated, growable, and owned by whoever holds it. When a `String` goes out of scope, the memory is freed.

The function signature makes this concrete:

```rust
pub fn repeat(s: &str, n: usize) -> String
```

The parameter is `&str` — we're borrowing the input, just reading it. The return type is `String` — we're creating something new and handing ownership to the caller. We couldn't return `&str` here because the string we're building doesn't exist anywhere before this function runs; there's nothing to reference.

This distinction — borrowed vs owned — runs through all of Rust. We'll keep coming back to it.

## Refactor

The naive loop works, but the standard library already has what we need. `&str` has a `repeat` method that does exactly this:

```rust
pub fn repeat(s: &str, n: usize) -> String {
    s.repeat(n)
}
```

Run `cargo test`:

```
running 1 test
test tests::repeat_a_string ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Same test, one line. Notice the compiler's four suggestions earlier didn't include this — `repeat` on `&str` lives in `std::str`, not in a separate module you'd import. It's just there on the type. This is worth knowing: when you find yourself writing a loop to accumulate a string, there's often a method that already does it. The standard library is worth getting familiar with.

Commit.

## Wrapping up

### Rust concepts introduced

- `String` vs `&str` — owned heap-allocated string vs borrowed string reference; you build with `String`, you borrow with `&str`
- `let mut` — variables are immutable by default; `mut` opts in to mutation
- `String::new()` and `push_str` — building a string incrementally
- `for _ in 0..n` — for loops and ranges
- `usize` — the type for counts and indices; can't be negative
- Implicit return — `result` with no semicolon returns the value

### Testing concepts

- Tests drive you to the right types — the return type `String` vs `&str` was forced by the requirement to build something new

### Coming up

We wrote the loop manually first, then replaced it with a standard library method. There's a whole family of tools in Rust for working with iterators — `map`, `filter`, `collect` and more. We'll get to those in a later chapter.

See [The TDD Cycle](../principles/tdd-cycle.md) for more on why each step matters.

### Further reading

- [`String` vs `&str`](https://doc.rust-lang.org/book/ch08-02-strings.html) — ownership explains why Rust has two string types; this chapter makes it click
- [Control flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) — `for` loops, ranges, and the iterator pattern that underpins them
