# Chapter Proposal: Structs

## Business requirement

We want to calculate the perimeter and area of rectangles.

## Rust features introduced

- `struct` — defining a named type with fields; grouping related data
- `f64` — floating point, introduced naturally by the geometry requirement
- `impl` — attaching methods to a struct; behaviour alongside data
- `&self` — borrowing the receiver in a method
- Field access with `.` — `rectangle.width`

## Deferred

- Ownership and moving structs — `&self` is introduced but the deeper story waits
- Traits (`Display`, `Debug`) — `#[derive(Debug)]` gets a mention but traits proper come later
- `f32` vs `f64` — brief mention only; `f64` is the default and that's enough

## Teaching order

1. Write test for `perimeter` using a `Rectangle` struct — fails, neither struct nor function exists; two compiler errors to work through
2. Define the struct with `width` and `height: f64` fields
3. Write standalone function `perimeter(r: Rectangle) -> f64` — green; introduces struct instantiation and field access
4. Refactor: move `perimeter` into `impl Rectangle` as a method taking `&self` — `rect.perimeter()`; tests still pass; explain `&self`
5. Add `area` test — red; add `area` method to `impl` block — green; reinforces the pattern

## Chapter shape

1. Introduction — business requirement
2. Write the test first — two compiler errors (no struct, no function); work through them in order
3. Make it pass — define struct, write standalone function
4. Methods — refactor into `impl` block, explain `&self`
5. Commit
6. Area — second red/green cycle using the same pattern
7. Wrapping up

## Teaching notes

- Two compiler errors in red step come sequentially — first `Rectangle` not found, then `perimeter` not found; good to address one at a time
- `pub` needed on struct fields — easy to miss; worth calling out explicitly
- Ownership contrast is the key teaching moment in the refactor: `r: Rectangle` takes ownership (can't use `r` after), `&self` borrows (can use `r` after)
- `impl` block is the natural home for methods — groups behaviour with data
- `self.width` / `self.height` inside `impl` mirrors `r.width` / `r.height` outside — same idea, different syntax
- `area` step felt very natural after `perimeter` — the pattern clicks quickly

## Status
teaching-complete
