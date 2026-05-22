# Test Behaviour, Not Implementation

This is one of the most important — and most commonly violated — principles in testing.

## The problem

Imagine you're building a square. You decide to implement it using two right-angled triangles. You write tests for the square (it has equal sides, it has right angles) and also tests for the triangles (angles sum to 180°, two triangles present, etc).

Later, someone realises squares can be made from two rectangles instead. She starts refactoring. Suddenly, tests for triangles start failing. She has to dig through them all to understand whether the behaviour she cares about — the square — is actually broken.

It isn't. The square still works. But the tests have **falsely elevated the importance of the implementation detail** (triangles) over the actual behaviour (a correct square).

This is the real reason developers say "unit tests get in the way of refactoring". It's not unit tests that are the problem — it's tests written at the wrong level.

## The principle

Test what a piece of code *does*, not *how* it does it.

- Test the public API, not private internals
- Test the outcome, not the steps taken to reach it
- Avoid asserting on which collaborators were called, in what order, with what arguments — unless that sequence *is* the behaviour you care about

## What is a "unit"?

A unit is not a function or a struct. A unit is a coherent piece of behaviour in your domain.

A `Wallet` is a unit. You test that you can deposit money, withdraw money, and check the balance. You don't test that internally it uses a `Vec<i64>` vs an `i32` field — that's an implementation detail.

If you change the internals and the tests don't break, you've succeeded. That's what good unit tests feel like.

## In Rust

Rust makes this natural. Mark your tests in a `#[cfg(test)]` module and test through the same public interface your callers would use. Don't reach into private fields or call private functions in tests.

```rust
pub struct Wallet {
    balance: i64,
}

impl Wallet {
    pub fn deposit(&mut self, amount: i64) {
        self.balance += amount;
    }

    pub fn balance(&self) -> i64 {
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_increases_balance() {
        let mut w = Wallet { balance: 0 };
        w.deposit(10);
        assert_eq!(w.balance(), 10);
    }
}
```

The test doesn't know or care how `balance` is stored. It tests the behaviour.

## See also

- [Listening to Your Tests](./listening-to-your-tests.md)
- [Separating Concerns](./separating-concerns.md)
