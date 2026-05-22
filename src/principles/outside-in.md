# Outside-In TDD

Outside-in TDD (also called "London School TDD" or "mockist TDD") is an approach to test-driven development that starts from the user's perspective and works inward through the system.

## The idea

Rather than starting with the lowest-level building blocks and assembling them upward, outside-in starts at the outermost behaviour — what a user or caller actually experiences — and uses that as the driver for what to build next.

You write a high-level test first. It fails, probably because the types and functions it needs don't exist yet. Then you build just enough of the internals to make it pass, guided by the tests you write at each layer.

## Why outside-in?

The alternative — inside-out development — risks building things that are technically correct but don't serve the real need. You might build a beautifully designed data structure that nobody calls, or a function with the wrong signature for how it's actually used.

Outside-in keeps the purpose in front of you. Every internal unit exists because an outer test required it.

## Acceptance tests and unit tests

Outside-in often involves two levels of tests:

1. **Acceptance tests** — high-level tests that describe end-to-end behaviour from a user's perspective. These are written first and are often slow or integration-heavy. They stay red for a while.
2. **Unit tests** — written as you implement each internal piece. These drive the design of the internals.

The acceptance test is the "outer loop". It defines *what* you're building. The unit tests are the "inner loop". They define *how* you build it.

## A note of caution

Outside-in TDD involves more mocking and interface design up front, because outer layers need to stand in for inner layers that don't exist yet. This is a legitimate trade-off, not a flaw.

However, it can be overused. If you mock everything and never let the real components interact, your tests won't catch integration problems. Use real implementations wherever practical; reach for mocks when the alternative is slow, non-deterministic, or has side effects you don't want in tests.

## In practice for this book

Most chapters start with a simple goal stated from the outside:

> "I want to be able to call `hello("World")` and get `"Hello, World"` back."

That's outside-in thinking. We write the test for the behaviour we want, then build what's needed to satisfy it.

## See also

- [The TDD Cycle](./tdd-cycle.md)
- [Incremental Development](./incremental-development.md)
