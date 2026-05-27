# Chapter Proposal: Traits

## Business requirement

We want to write a function that prints the area of any shape — rectangle, circle, or anything else we might add later.

## The problem this chapter solves

At the end of the structs chapter, `Rectangle` and `Circle` both have an `area()` method, but they have no connection in the type system. There is no way to write a single function that accepts either. If you tried:

```rust
fn print_area(shape: ???) {
    println!("Area is {}", shape.area());
}
```

There's no type you can put where `???` is. Traits solve this.

## Rust features introduced

- `trait` — defining a shared interface: a named set of method signatures that a type can implement
- `impl Trait for Type` — implementing a trait for a concrete type
- Trait as a parameter type: `fn print_area(shape: &impl Shape)` — the simplest way to accept any type that implements a trait
- The compiler enforcing the contract: if a type claims to implement a trait but is missing a method, it won't compile

## Deferred

- `dyn Trait` / trait objects — dynamic dispatch; deferred to a later chapter
- Generics (`fn foo<T: Trait>`) — equivalent to `impl Trait` syntax but more explicit; mention briefly, defer detail
- Trait bounds with multiple traits (`T: Shape + Debug`)
- Default method implementations in traits
- Traits from the standard library (`Display`, `Debug`, `Iterator`) — mentioned in wrapping up, covered properly later

## Teaching order

1. Establish the problem: write a test for a `print_area` function that takes a `Rectangle`. Make it pass.
2. Now try to write a second test passing a `Circle` — same function, different type. Hit the compiler error.
3. Introduce `trait Shape` with a single method `area(&self) -> f64`. This is the contract.
4. Implement `Shape` for `Rectangle` and `Circle` — `impl Shape for Rectangle`, `impl Shape for Circle`.
5. Update `print_area` to take `shape: &impl Shape`. All tests pass.
6. Add a `Triangle` struct to show how easy it is to extend: implement `Shape`, add a test, done.
7. Wrapping up — mention `dyn Trait` exists; standard library traits teaser.

## Chapter shape

1. **The problem** — `print_area` for Rectangle works; trying to pass Circle fails; what type do we use?
2. **Defining a trait** — `trait Shape`; the contract
3. **Implementing the trait** — `impl Shape for Rectangle`, `impl Shape for Circle`; compiler enforces the contract
4. **`&impl Shape`** — updating the function signature; all tests pass
5. **Adding a Triangle** — extending is cheap; the trait does the work
6. **Wrapping up**

## Teaching notes (from planning)

- The problem must be felt before the solution is shown — write the `print_area(r: Rectangle)` test first, get it green, then try to add the circle test and hit the wall
- `&impl Shape` syntax is the right place to start — it's the simplest and most readable; `dyn` and generics come later
- `impl Shape for Rectangle` mirrors `impl Rectangle` from the previous chapter — same keyword, different purpose; worth noting explicitly
- Triangle step reinforces that the trait is the stable contract; new shapes don't require changing existing code
- The compiler error when a trait implementation is incomplete (`error[E0046]: not all trait items implemented`) is a good teaching moment — show it
- `fn describe_area(shape: &impl Shape) -> String` returning `format!("This shape has an area of {}", shape.area())` — fully testable, no stdout complexity

## Teaching notes (from session)

- This chapter continues in the `structs` crate — no new crate needed
- `describe_area` started taking `Rectangle` by value; when `&impl Shape` was introduced the compiler gave a perfect "consider borrowing here" hint — show this error in the prose
- Float formatting: `{}` drops the decimal point for whole numbers (e.g. `100` not `100.0`) — author decided to test for `"100"` without decimal; prose should acknowledge this briefly
- Author tried `rec: Shape` as the parameter type (without `&` or `impl`) — Rust can't size a bare trait; the compiler suggests `dyn`; use this as the moment to introduce `&impl Shape` as the idiomatic starting point
- `impl Shape for Rectangle` initially delegated to `self.area()` — works but is duplication; moved the implementation directly into the trait impl and removed `area` from `impl Rectangle`/`impl Circle`
- The Go implicit interface comparison came up — worth a sentence in wrapping up: explicit `impl Trait for Type` means you can always see exactly which traits a type satisfies by reading the file; Go's implicit satisfaction means less ceremony; both are deliberate tradeoffs
- Table tests: author's instinct was an array of tuples — right idea, wrong because array elements must be the same type; `Vec<(&str, &dyn Shape, &str)>` is the fix; this is where `dyn Shape` is introduced naturally
- `dyn Shape` vs `&impl Shape`: `&impl Shape` is resolved at compile time (one concrete type); `&dyn Shape` is resolved at runtime (any type, mixed in a collection); `describe_area` updated to take `&dyn Shape` to work with the table test
- Without named cases, a failing table test only shows the values — not which case failed; adding a `&str` name and using `assert_eq!(..., "failed for {}", name)` fixes this; author confirmed the improvement
- Final code: `structs/src/lib.rs` — `Shape` trait, `impl Shape for Rectangle/Circle/Triangle`, `describe_area(&dyn Shape)`, table test with named cases
- Individual `description_of_rectangle/circle/triangle` tests kept alongside the table test during teaching — writer can remove them as redundant since the table test covers the same ground

## Status
teaching-complete
