# Chapter Proposal: Strings and Iteration

## Business requirement

We want a `repeat` function that returns a `String` consisting of a word repeated n times.
E.g. `repeat("na", 4)` → `"nananana"`.

## Rust features introduced

- `String` vs `&str` — forced by the requirement to build and return an owned string; we've used both but never explained the difference
- `for` loops and ranges (`0..n`) — the naive accumulator approach
- `String::new()` and `push_str` — building a string incrementally
- `usize` — the natural type for a count or index
- `.repeat()` — the idiomatic one-liner, arrived at by refactoring from the naive version

## Deferred

- Iterators, `map`, `collect` — too much for this chapter; worth a forward reference at the end
- `format!` for string building — already seen, not the focus here

## Teaching order

1. Write test: `assert_eq!(repeat("na", 4), "nananana")` — fails, function doesn't exist yet; compiler error is the teaching moment
2. Naive implementation: `String::new()` + `for _ in 0..n` + `push_str` — green; teaches `String`, `for`, ranges, `usize`
3. Explain `String` vs `&str` properly — return type is `String` (owned, heap-allocated), parameter is `&str` (borrowed reference); why each
4. Refactor: replace the loop body with `.repeat(n)` from the standard library — same tests pass, one line; teaches that the stdlib is worth knowing

## Chapter shape

1. Introduction — business requirement
2. Write the test first — compiler error, explain it
3. Make it pass — naive loop, explain `String`, `for`, `usize`
4. String vs &str — explain the distinction properly now that we've seen both
5. Refactor — `.repeat()`, re-run tests
6. Wrapping up — concepts introduced

## Status
planning-approved
