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

## Teaching notes

- The compiler suggested four `repeat` imports when the function didn't exist: `std::array::repeat`, `std::io::repeat`, `std::iter::repeat`, `core::array::repeat` — worth showing verbatim; `std::iter::repeat` (infinite iterator) is interesting to mention by name and defer
- `mut` came up naturally — variables are immutable by default in Rust; removing `mut` from `result` is a good suggested experiment for the reader
- `usize` vs `i32` — the "why not i32?" question is worth addressing; usize is the type for counts and indices, can't be negative, matches what ranges produce
- `s.repeat(n)` is a method on `&str` (in `std::str`) — notably absent from the compiler's four suggestions, which makes it a nice discovery moment during refactor
- Implicit return on `result` (no semicolon) ties back to Hello World and Integers — good to call out the pattern again

## Status
complete
