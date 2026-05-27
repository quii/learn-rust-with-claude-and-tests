# Chapter Proposal: Structs, Methods and Interfaces

## Business requirement

We want some geometry code to calculate the perimeter and area of shapes.

## Rust features introduced

- `f64` — floating point; natural fit for geometry
- `struct` — defining a named type with fields; introduced as a *refactor* to fix ambiguity in plain functions
- Field access with `.` — `rectangle.width`
- `pub` on struct and fields — required for visibility outside the module
- `impl` — attaching methods to a type; motivated by the naming collision when Circle arrives
- `&self` — borrowing the receiver in a method
- Table-driven tests with a `Vec` of tuples — introduced in the refactor step after Circle

## Deferred

- Traits — the natural next step after this chapter; teased in Wrapping Up
- `dyn Trait` / generics over traits — deferred to the traits chapter
- `f32` vs `f64` — brief mention only
- Ownership and moving structs — `&self` is introduced but the deeper story waits

## Teaching order

1. Write `perimeter(width: f64, height: f64) -> f64` — plain function, no struct. Red → green.
2. Write `area(width: f64, height: f64) -> f64` — same shape. Red → green. ("Try it yourself" beat.)
3. Refactor: introduce `Rectangle` struct. Motivation: an unwary caller could pass triangle dimensions and get a wrong answer. Plain floats don't encode the intent. Update tests and functions to take `Rectangle`. Compiler guides the change.
4. Add `Circle` struct with `area`. Attempt to write `fn area(c: Circle)` — naming collision with existing `area(r: Rectangle)`. Rust won't allow two free functions with the same name and different types. This is the problem that motivates methods.
5. Refactor both to `impl` blocks: `rectangle.area()`, `circle.area()`. Explain `&self`.
6. Add `perimeter` as a method on `Rectangle` for completeness.
7. Refactor tests to table-driven style: `Vec` of structs, iterate with `for`.
8. Wrapping up — traits teaser.

## Chapter shape

1. **Perimeter** — plain function; red/green; `f64` introduced
2. **Area** — "have a go" beat; reader writes it themselves; green
3. **Refactor to Rectangle** — struct introduced as solution to ambiguity; compiler-guided refactor
4. **Circle** — new shape, new struct; naming collision exposed
5. **Methods** — `impl` blocks solve the collision; `&self` explained; ownership contrast noted
6. **Table-driven tests** — refactor the tests; `Vec` of test cases; iterate
7. **Wrapping up** — concepts listed; traits chapter teased

## Teaching notes

- The plain-functions-first step is essential: the reader must *feel* the problem before the struct is introduced
- "An unwary developer might pass triangle dimensions" — say this explicitly; it's the moment the abstraction earns its place
- Naming collision in Rust is a compiler error (`error[E0428]: the name 'area' is defined multiple times`) — let the reader hit it
- `&self` contrast: `r: Rectangle` takes ownership (moves), `&self` borrows — worth a short demonstration
- Table-driven tests: use a `Vec` of anonymous structs or a `Vec` of tuples; lean toward tuples for simplicity; named struct if it aids clarity
- Traits teaser at the end: "right now Rectangle and Circle have no connection — a function can't accept both. That's what traits are for."

## Teaching notes

- Author naturally skipped the "minimal failing implementation" step (returning 0) that LGWT uses — went straight to the correct implementation. Fine for this book's pace.
- Typo in first test name (`rectagnle`) and function-inside-test-module mistake — both worth a brief mention in prose as common early stumbles, not dwelt on.
- `pub` on struct/fields was not needed during teaching because tests are in the same crate — prose should note this and explain when `pub` is needed (cross-crate use), but don't make it the focus.
- Struct literal syntax (`Rectangle { width: 10.0, height: 10.0 }`) is not obvious to newcomers — show it explicitly, don't ask them to guess.
- The naming collision error (`area` redefined) landed perfectly — author predicted it before running, then hit it. Paste the exact error in the prose.
- Author's instinct on incremental TDD was sharp: fix Circle (the broken thing) first without touching Rectangle (the working thing). Make this explicit in the prose as the disciplined approach.
- `impl` was genuinely new — showed the syntax with explanation before asking author to use it.
- `std::f64::consts::PI` — author didn't know where to find it; show the `use` line explicitly in the prose.
- Author chose `self.radius * self.radius * PI` over `PI * self.radius.powi(2)` — keep the simpler form in the prose.
- Free functions `perimeter` and `area` were still in the file after introducing methods — easy to forget; prose should remind the reader to delete them.
- Table-driven tests deliberately skipped — not a distraction for this chapter; may appear in a later chapter.
- Author asked good questions about orphan rule and traits at the end — answered briefly; traits chapter is next.
- Final code: `use std::f64::consts::PI` at top; `impl Rectangle` with `area` and `perimeter`; `impl Circle` with `area`; structs defined below impls (Rust allows this); three tests all passing.

## Status
teaching-complete
