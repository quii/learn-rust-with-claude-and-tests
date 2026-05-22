# Separating Concerns

Good code separates what a thing *does* from *how it interacts with the world*.

This is one of the oldest principles in software design, and it applies with full force when writing testable code.

## Domain vs side effects

Consider a function that greets someone by name:

```rust
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

This is pure. It takes input, returns output. You can test it with no setup:

```rust
#[test]
fn greets_by_name() {
    assert_eq!(greet("World"), "Hello, World!");
}
```

Now consider this version:

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

You can't easily test this without capturing stdout. The greeting logic and the output are tangled together.

**Separate them.** The business logic (what to say) belongs in pure functions. The side effects (printing, writing to a file, sending a network request) belong at the boundary of the system, called from `main` or an application layer.

## What counts as a "side effect"?

- Writing to stdout or stderr
- Reading from or writing to the file system
- Network I/O
- Reading the clock or generating random numbers
- Mutating global state

These are all things that make tests slow, flaky, or hard to set up. Keep them at the edges.

## Dependency injection

When your code needs to interact with the outside world, inject the mechanism rather than hard-coding it. Pass a writer instead of calling `println!`. Pass a clock interface instead of calling `std::time::Instant::now()`.

This lets tests substitute fast, controlled fakes for slow, uncontrollable real things.

In Rust, this typically means accepting a generic type parameter or trait object:

```rust
use std::io::Write;

fn greet<W: Write>(writer: &mut W, name: &str) {
    writeln!(writer, "Hello, {}!", name).unwrap();
}

#[test]
fn greet_writes_to_writer() {
    let mut output = Vec::new();
    greet(&mut output, "World");
    assert_eq!(output, b"Hello, World!\n");
}
```

In the real application, you'd call `greet(&mut std::io::stdout(), "World")`. In the test, you use a `Vec<u8>` — fast, in-memory, and fully inspectable.

## The payoff

When concerns are separated:
- Tests are fast and easy to write
- The core logic can be understood in isolation
- Side effects are explicit and localised
- Refactoring the implementation doesn't break the tests

This isn't just about testing. Separated code is easier to read, reason about, and change — for any reason.

## See also

- [Test Behaviour, Not Implementation](./test-behaviour-not-implementation.md)
- [Listening to Your Tests](./listening-to-your-tests.md)
