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

`println!` is a **macro**, not a function. The `!` is the giveaway. Macros in Rust are a way of writing code that writes code — we won't go deep on them now, but you'll see them frequently. For the moment, treat `println!` as "print this to the terminal with a newline".

`fn main()` is the entry point of every Rust program. Execution starts here.

## How to test

How do you test this? The problem is that `println!` is a **side effect** — it prints to the terminal, which is the outside world. Side effects are hard to test because you'd have to capture what gets printed and inspect it.

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
        assert_eq!(greet("World"), "Hello, World!");
    }
}
```

Don't write the `greet` function yet. Run the tests:

```
cargo test
```

```
error[E0425]: cannot find function `greet` in this scope
  --> src/main.rs:11:20
   |
11 |         assert_eq!(greet("World"), "Hello, World!");
   |                    ^^^^^ not found in this scope
```

This is the **red** step. The compiler is telling you exactly what's missing. In Rust, the compiler is unusually helpful — learning to read its output is one of the most valuable skills you'll develop.

Notice that the test didn't just fail — it didn't compile. In a statically typed language, a type error _is_ a test failure. The compiler caught a problem before anything ran.

### A few new concepts

**`#[cfg(test)]`** tells the compiler to only include this module when running tests. The test code doesn't end up in your production binary.

**`mod tests`** creates a module — a namespace — for the tests. The name `tests` is conventional but not required.

**`use super::*`** brings everything from the parent module into scope. This is how the test can see `greet` once we define it.

**`#[test]`** marks a function as a test. `cargo test` finds and runs all functions marked this way.

**`assert_eq!`** is another macro. It checks that two values are equal and fails the test with a clear message if they aren't.

## Make it pass

Add the `greet` function above `main`:

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

A couple of things to note:

`&str` is a **string slice** — the standard type for borrowed string data in Rust. When a function just needs to read a string, `&str` is usually the right choice. We'll explore Rust's string types properly in a later chapter.

`-> String` declares that the function returns an owned `String`.

`format!` is like `println!` but returns a `String` instead of printing it. This is exactly the separation we wanted: the logic produces a value, `main` decides what to do with it.

Run the tests:

```
cargo test
```

```
running 1 test
test tests::test_greet ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Green. Notice how fast that was — finished in 0.00s. A fast feedback loop matters. You want tests to be so quick that running them feels like no effort at all.

## Refactor

Now update `main` to use `greet`:

```rust
fn main() {
    println!("{}", greet("World"));
}
```

Run both to confirm nothing broke:

```
cargo run
```

```
Hello, World!
```

```
cargo test
```

```
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The behaviour hasn't changed. The code is better. That's a refactor.

Commit before going further:

```
git add .
git commit -m "Hello World: greet function with first passing test"
```

## Hello, YOU

Now that we have a test, we can iterate safely.

Our next requirement: the greeting should address a specific person by name. Let's capture that as a test first.

We have an existing test. Rather than replace it, we'll introduce subtests using Rust's built-in test organisation. Update the test module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_world_by_default() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn greets_a_person_by_name() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }
}
```

Run the tests. The second one will pass immediately — our current implementation already handles it. That's fine; the test still has value as a specification.

Now introduce a new requirement: when called with an empty string, `greet` should default to `"World"`.

Write the test first:

```rust
#[test]
fn greets_world_when_name_is_empty() {
    assert_eq!(greet(""), "Hello, World!");
}
```

Run it. It will fail:

```
thread 'tests::greets_world_when_name_is_empty' panicked at 'assertion `left == right` failed
  left: "Hello, !"
 right: "Hello, World!"'
```

Good. A clear failure message. Now make it pass:

```rust
fn greet(name: &str) -> String {
    let name = if name.is_empty() { "World" } else { name };
    format!("Hello, {}!", name)
}
```

A few new things here:

`let name = ...` introduces a new binding that shadows the parameter. Rust allows this — it's idiomatic to rebind a variable with a refined value rather than mutate it.

`if name.is_empty() { "World" } else { name }` — in Rust, `if` is an expression, not just a statement. It produces a value. This whole line is an expression that evaluates to either `"World"` or the original `name`.

Run the tests:

```
cargo test
```

```
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

All green. Commit.

## Wrapping up

We've covered a lot of ground in a short program.

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

We followed the cycle: write a failing test, write the minimum code to make it pass, refactor. In this case the steps were small — almost trivially so. That's the point. Small steps, always green, never lost.

See [The TDD Cycle](../principles/tdd-cycle.md) for more on why each step matters.
