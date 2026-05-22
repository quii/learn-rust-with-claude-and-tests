# Listening to Your Tests

Tests are not just a safety net. They give you feedback about your design.

When tests are painful to write, that pain is telling you something. Learn to listen.

## Common signals and what they mean

### "This is hard to test without setting up a lot of state"

Your component has too many dependencies, or its dependencies are too concrete. Consider injecting abstractions instead of constructing things internally.

### "I have to change many tests when I refactor the internals"

Your tests are testing implementation details rather than behaviour. See [Test Behaviour, Not Implementation](./test-behaviour-not-implementation.md).

### "I need to mock everything to test anything"

Your code is doing too much, or concerns are mixed. A function that both fetches data and formats it is harder to test than two functions that do each separately. See [Separating Concerns](./separating-concerns.md).

### "My tests are slow"

Something in the code under test is doing I/O, sleeping, or hitting a real service. Extract the slow part behind an interface and inject a fast substitute in tests.

### "I can't see how to write a test for this"

This usually means the behaviour isn't clearly defined, or the code is entangled with things it shouldn't be. It's rarely a testing problem — it's a design problem that the test is exposing.

## The key insight

If you view testing as an afterthought — something you do after the code is "done" — you lose the design feedback. The tests are supposed to push back. They're supposed to make you rethink how things are structured.

TDD makes this feedback fast. You feel the friction before you've written too much code to change comfortably.

## Rust-specific notes

Rust's type system and ownership model are unusually good at this. If something is hard to test in Rust, it's often because ownership boundaries aren't clear, or because a type is doing too much. The borrow checker will refuse to let you take shortcuts that would create hidden coupling in other languages.

Take that friction seriously. It's almost always pointing at something real.

## See also

- [The TDD Cycle](./tdd-cycle.md)
- [Test Behaviour, Not Implementation](./test-behaviour-not-implementation.md)
- [Separating Concerns](./separating-concerns.md)
