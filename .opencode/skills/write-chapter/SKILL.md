---
name: write-chapter
description: Use when writing or editing a chapter for Learn Rust with Tests. Covers the full workflow from coding incrementally through TDD, capturing real output, writing prose, and verifying the chapter as a reader.
---

# Writing a Chapter for Learn Rust with Tests

This skill governs the full process for writing a chapter. It exists because the integrity of the book depends on the author having genuinely followed the journey — not generated the end state and worked backwards.

## The guiding principle

The book teaches TDD by example. If the chapter is written by producing the final code first and then narrating around it, the writing will feel false, steps will be missing or wrong, and the compiler output will be reconstructed rather than real. Readers notice. Don't do it.

## The process

### 1. Establish the requirement

Every chapter starts with a plain-English business requirement. State it clearly before writing any code or tests. Example: "We want a `greet` function that returns a personalised greeting."

### 2. Create the exercise project

```sh
cargo new <chapter-name>
cd <chapter-name>
```

Show the generated files and explain them briefly. Run `cargo run` to show the default output before touching anything.

### 3. Follow the TDD cycle — incrementally

For each requirement:

1. **Write the test first** — do not write the implementation yet
2. **Run `cargo test`** and capture the actual output
3. **Make the minimum change** to move forward (fix a compile error, make a test pass)
4. **Run `cargo test` again** and capture the actual output
5. **Repeat** until green
6. **Refactor** if needed, running tests after each change
7. **Commit** at green before moving to the next requirement

Never skip ahead to the end state. Each step must be run and observed before writing about it.

### 4. Capture real output verbatim

Every compiler error, warning, and test result in the prose must come from actually running the command. Copy and paste from the terminal. Do not reconstruct from memory or approximate what the output "probably looks like".

This includes:
- Compiler errors (`error[E0425]: ...`)
- Warnings (`warning: unused variable: ...`)
- Test output (`test result: ok. 1 passed; ...`)
- `cargo run` output

If the output is long, trim the irrelevant parts but do not alter what remains.

### 5. Compiler errors are teaching moments

Rust's compiler errors are unusually informative. When a step produces an error, don't rush past it — name what it's telling the reader and explain why it matters. The reader is learning to use the compiler as a collaborator.

### 6. Write the prose around the journey

The prose narrates what just happened and why. The rhythm is:

- State the next requirement or observation
- Show the code change
- Show the real output
- Explain what the output means and what to do next
- Repeat

Voice: teaching, not documenting. Brisk, not padded. Trust the reader to be intelligent. Link to the `principles/` docs rather than re-explaining TDD concepts inline.

### 7. Chapter structure

Every chapter follows this shape:

1. **Introduction** — the business requirement, brief context
2. **How to test** — why we separate concerns before writing any tests
3. **Write the test first** — red step, with real compiler/test output; explain new testing concepts as they appear
4. **Make it pass** — green step, minimum code, real output; explain new Rust concepts as they appear
5. **Refactor** — if applicable, with tests re-run to confirm
6. **Commit** — explicitly tell the reader to commit at green
7. Repeat steps 3–6 for each new requirement
8. **Wrapping up** — two lists: Rust concepts introduced, TDD/testing concepts introduced; link to relevant principles pages

### 8. Proof-read as a reader

After writing the chapter, verify it from a blank slate:

1. Reset the exercise file to its `cargo new` default
2. Follow the chapter's instructions exactly, making only the changes described
3. Run every command shown
4. Confirm that every piece of output in the prose matches what the terminal actually produces

If anything doesn't match — wrong error message, missing step, output that changed — fix the prose or code before committing.

### 9. Commit discipline

- Commit at every green point, before adding the next requirement
- The final commit message for a chapter should be: `<Chapter Name>: chapter prose and code following TDD steps incrementally`
- The `target/` directory is gitignored — never commit build output

## What goes where

- Exercise code: `<chapter-name>/src/main.rs` (or `lib.rs` for library crates)
- Chapter prose: `src/fundamentals/<chapter-name>.md`
- New chapters must be added to `src/SUMMARY.md`

## What to avoid

- Do not write the final state of the code and then write prose around it
- Do not approximate or reconstruct compiler output — always use real output
- Do not re-explain TDD principles that are already covered in `src/principles/` — link instead
- Do not introduce more than one or two new Rust concepts per step without acknowledging the load on the reader
- Do not skip the proof-reading step
