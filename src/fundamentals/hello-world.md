# Hello, World

**[You can find all the code for this chapter here](https://github.com/TODO/learn-rust-with-tests/tree/main/hello-world)**

It is traditional for your first program in a new language to be [Hello, World](https://en.m.wikipedia.org/wiki/%22Hello,_World!%22_program).

In Rust, we use `cargo` to create and manage projects. Run the following:

```
cargo new hello-world
cd hello-world
```

`cargo` is Rust's build tool and package manager. `cargo new` creates a project with a standard structure. Have a look at what it generated:

```
src/main.rs
Cargo.toml
```

`Cargo.toml` describes your project — its name, version, and dependencies. `src/main.rs` is where your code lives. Open it and you'll see:

```rust
fn main() {
    println!("Hello, world!");
}
```

Run it:

```
cargo run
```

```
Hello, world!
```

Two things worth noting before we move on.

`println!` is a **macro**, not a function. The `!` is the giveaway. For now, treat it as "print this to the terminal with a newline".

`fn main()` is the entry point of every Rust program. Execution starts here.

## How to test

How do you test this? The problem is that `println!` is a **side effect** — it prints to the terminal, which is the outside world. Side effects are hard to test.

The better approach is to separate what we want to _say_ from the act of _saying it_. The greeting logic belongs in a pure function that returns a value. The printing belongs in `main`.

This is a pattern we'll return to throughout this book. See [Separating Concerns](../principles/separating-concerns.md) for the deeper reasoning.

## Write the test first

Add the following to the bottom of `src/main.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet(), "Hello, World!");
    }
}
```

Don't write `greet` yet. Run the tests:

```
cargo test
```

```
error[E0425]: cannot find function `greet` in this scope
  --> src/main.rs:11:20
   |
11 |         assert_eq!(greet(), "Hello, World!");
   |                    ^^^^^ not found in this scope
```

This is the **red** step. The compiler is telling you exactly what's missing.

### A few new concepts

**`#[cfg(test)]`** tells the compiler to only include this module when running tests. The test code doesn't end up in your production binary.

**`mod tests`** creates a module — a namespace — for our tests. The name `tests` is conventional.

**`use super::*`** brings everything from the parent module into scope, so the test can see `greet` once we define it.

**`#[test]`** marks a function as a test. `cargo test` finds and runs all functions marked this way.

**`assert_eq!`** checks that two values are equal and fails the test with a clear message if they aren't.

## Make it pass

Add the `greet` function above `main`:

```rust
fn greet() -> String {
    format!("Hello, World!")
}
```

`-> String` declares that the function returns an owned `String`. `format!` works like `println!` but returns a `String` instead of printing — exactly the separation we wanted.

Update `main` to use it:

```rust
fn main() {
    println!("{}", greet());
}
```

Run the tests:

```
cargo test
```

```
running 1 test
test tests::test_greet ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Green. And fast. A tight feedback loop matters — you want running tests to feel like no effort at all.

This is a good moment to commit before we add the next requirement.

## Hello, YOU

Now that we have a test, we can iterate safely.

Our next requirement: the greeting should address a specific person. Let's write the test first:

```rust
#[test]
fn greets_a_person_by_name() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
}
```

Run the tests:

```
error[E0061]: this function takes 0 arguments but 1 argument was supplied
  --> src/main.rs:15:20
   |
15 |         assert_eq!(greet("Alice"), "Hello, Alice!");
   |                    ^^^^^ -------
   |                          argument unexpected
```

The compiler has done our TODO list for us. `greet` doesn't take any arguments yet. Let's fix that — but minimally. Change the signature to accept a name:

```rust
fn greet(name: &str) -> String {
    format!("Hello, World!")
}
```

`&str` is a **string slice** — the standard type for borrowed string data in Rust. When a function just needs to read a string, `&str` is usually the right choice. We'll explore Rust's string types properly in a later chapter.

The function now compiles, but the tests will tell us it's wrong:

```
cargo test
```

```
thread 'tests::greets_a_person_by_name' panicked at 'assertion failed: `(left == right)`
  left: "Hello, World!",
 right: "Hello, Alice!"'
```

Good — a clear failure message. Now make it pass:

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

```
cargo test
```

```
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

But wait — our first test is now broken in spirit even if it passes. It currently calls `greet("World")`, which works, but `"World"` is just a string we happened to hardcode. What we actually want is: if no meaningful name is given, default to `"World"`. Let's make that requirement explicit.

## Default behaviour

Update the first test to pass an empty string instead:

```rust
#[test]
fn greets_world_by_default() {
    assert_eq!(greet(""), "Hello, World!");
}
```

Run the tests. This one will now fail:

```
thread 'tests::greets_world_by_default' panicked at 'assertion failed: `(left == right)`
  left: "Hello, !",
 right: "Hello, World!"'
```

Now make it pass:

```rust
fn greet(name: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    format!("Hello, {}!", name)
}
```

A couple of things here:

`let name = ...` introduces a new binding that **shadows** the parameter. Rust allows — and encourages — this rather than mutating a variable.

`if name.is_empty() { "World" } else { name }` — in Rust, `if` is an **expression**, not just a statement. It produces a value. The whole line evaluates to either `"World"` or the original `name`.

Run the tests:

```
cargo test
```

```
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

All green. Update `main` to reflect the new signature:

```rust
fn main() {
    println!("{}", greet("World"));
}
```

Commit.

## Wrapping up

We've covered a lot of ground in a small program.

### Rust concepts introduced

- `cargo new` — create a project
- `fn` — declare a function
- `&str` and `String` — Rust's two main string types (more on this later)
- `format!`, `println!` — macros for working with strings
- `if` as an expression
- Variable shadowing with `let`

### Testing concepts

- `#[cfg(test)]` and `mod tests` — how Rust isolates test code
- `#[test]` — marking a function as a test
- `assert_eq!` — asserting equality
- The value of watching a test fail before making it pass

### The TDD process

We followed the cycle deliberately: write a failing test, write the minimum code to make it pass, refactor. Notice how the compiler errors guided each step — in Rust, the compiler is a collaborator, not an obstacle. Learning to read what it's telling you is one of the most valuable things you can do early on.

See [The TDD Cycle](../principles/tdd-cycle.md) for more on why each step matters.
