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
warning: unused import: `super::*`
error[E0425]: cannot find function `greet` in this scope
  --> src/main.rs:11:20
   |
11 |         assert_eq!(greet(), "Hello, World!");
   |                    ^^^^^ not found in this scope
```

This is the **red** step. The compiler is telling you exactly what's missing. There's also a warning about the unused `use super::*` import — once `greet` exists, that import will be used and the warning will disappear.

### A few new concepts

**`#[cfg(test)]`** tells the compiler to only include this module when running tests. The test code doesn't end up in your production binary.

**`mod tests`** creates a module — a namespace — for our tests. The name `tests` is conventional.

**`use super::*`** brings everything from the parent module into scope, so the test can see `greet` once we define it.

**`#[test]`** marks a function as a test. `cargo test` finds and runs all functions marked this way.

**`assert_eq!`** checks that two values are equal and fails the test with a clear message if they aren't.

## Make it pass

Add the `greet` function above `main`, and update `main` to use it:

```rust
fn main() {
    println!("{}", greet());
}

fn greet() -> String {
    format!("Hello, World!")
}
```

`-> String` declares that the function returns an owned `String`. `format!` works like `println!` but returns a `String` instead of printing — exactly the separation we wanted.

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
  --> src/main.rs:20:20
   |
20 |         assert_eq!(greet("Alice"), "Hello, Alice!");
   |                    ^^^^^ ------- unexpected argument of type `&'static str`
```

The compiler has done our TODO list for us. `greet` needs to accept a name. Let's add the parameter — but do the minimum: just change the signature, don't use `name` yet:

```rust
fn greet(name: &str) -> String {
    format!("Hello, World!")
}
```

`&str` is a **string slice** — the standard type for borrowed string data in Rust. When a function just needs to read a string, `&str` is usually the right choice. We'll explore Rust's string types properly in a later chapter.

Run the tests:

```
error[E0061]: this function takes 1 argument but 0 arguments were supplied
 --> src/main.rs:2:20
  |
2 |     println!("{}", greet());
  |                    ^^^^^-- argument #1 of type `&str` is missing

error[E0061]: this function takes 1 argument but 0 arguments were supplied
  --> src/main.rs:15:20
   |
15 |         assert_eq!(greet(), "Hello, World!");
   |                    ^^^^^-- argument #1 of type `&str` is missing

warning: unused variable: `name`
```

Two things to notice. First: adding the parameter broke *two* places — `main` and the first test — both of which were calling `greet()` with no arguments. Second: Rust warns you about the unused `name` variable. It won't let you silently ignore things. Fix both call sites by passing `"World"` for now:

```rust
fn main() {
    println!("{}", greet("World"));
}
```

```rust
#[test]
fn test_greet() {
    assert_eq!(greet("World"), "Hello, World!");
}
```

Run the tests again:

```
warning: unused variable: `name`

test tests::test_greet ... ok
test tests::greets_a_person_by_name ... FAILED

---- tests::greets_a_person_by_name stdout ----
assertion `left == right` failed
  left: "Hello, World!"
 right: "Hello, Alice!"
```

It compiles. The first test passes, the second tells us exactly what's wrong. Make it pass by using `name`:

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

```
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Both green, warning gone. But look at the first test — it's passing `"World"` as an argument. That works, but it feels off. We're not really testing that the function greets a person named "World" — we're using it as a stand-in for "no name given". That intent isn't expressed anywhere. Let's make it explicit.

## Default behaviour

Update the first test to pass an empty string instead, and rename it to reflect the intent:

```rust
#[test]
fn greets_world_by_default() {
    assert_eq!(greet(""), "Hello, World!");
}
```

Run the tests:

```
test tests::greets_world_by_default ... FAILED

---- tests::greets_world_by_default stdout ----
assertion `left == right` failed
  left: "Hello, !"
 right: "Hello, World!"
```

Good — a clear failure. Now make it pass:

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
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

All green. Commit.

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
