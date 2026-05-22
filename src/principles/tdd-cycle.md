# The TDD Cycle

Test-Driven Development is a discipline, not just a technique. It shapes how you think about software, not just how you write it. The cycle has three steps, and each step matters for a specific reason.

## Red

**Write a failing test.**

Before writing any production code, write a test that describes a small piece of behaviour you want. Run it. Watch it fail.

This step is often skipped by newcomers who feel it's wasteful — "I know it'll fail, why bother running it?" The reason is this: a test that you haven't seen fail is a test you can't fully trust. You need to confirm:

- The test actually runs
- The failure message is clear and meaningful
- The test is testing what you think it's testing

A red test is a precise, executable description of a requirement.

## Green

**Write the minimum code to make the test pass.**

Not good code. Not extensible code. The _minimum_. This is a constraint worth taking seriously.

When you're in the green step, your only job is to satisfy the test. This keeps your focus narrow and your steps small. If you find yourself writing more than you need to make the current test pass, stop — you're speculating about future requirements.

> "Make it work, make it right, make it fast." — Kent Beck

## Refactor

**Clean up the code, with your tests as a safety net.**

Now that the behaviour is correct and verified, improve the code. Rename things. Extract functions. Remove duplication. This is the step where design happens — but safely, because you have a test suite telling you if you've broken anything.

Crucially: **refactoring must not change behaviour**. If your tests pass before and after, you've refactored. If a test changes or you change what the code does, you've done something else.

## The cycle in practice

```
Write a failing test  →  Make it pass  →  Refactor  →  repeat
```

The loop should be fast. Seconds, not minutes. If you find yourself in a long green or refactor phase without running tests, something has gone wrong. The tightness of the feedback loop is what gives TDD its power.

You'll know you've internalised TDD when a long stretch without a green test starts to feel uncomfortable — like a long time between saves in a document you're writing.

## Why not write all the tests first?

TDD is not "write all your tests and then write all your code". That defeats the purpose. Each test should be written immediately before the code that makes it pass. The tests drive the design of the code, one small step at a time.

## See also

- [Incremental Development](./incremental-development.md)
- [Listening to Your Tests](./listening-to-your-tests.md)
