---
name: write-chapter
description: Use when writing or editing a chapter for Learn Rust with Tests. Covers the full workflow from coding incrementally through TDD, capturing real output, writing prose, and verifying the chapter as a reader.
---

# Writing a Chapter for Learn Rust with Tests

This skill governs the full process for writing a chapter. It exists because the integrity of the book depends on the author having genuinely followed the journey — not generated the end state and worked backwards.

## The guiding principle

The book teaches TDD by example. If the chapter is written by producing the final code first and then narrating around it, the writing will feel false, steps will be missing or wrong, and the compiler output will be reconstructed rather than real. Readers notice. Don't do it.

## The three phases

Every chapter goes through three phases in order. Do not skip or merge them.

### Phase 1: Planning

Before any code or teaching, agree the shape of the chapter:

1. The business requirement — what the code will do by the end
2. The Rust features that will be introduced, and why each one earns its place
3. The teaching order — what gets introduced first, what compiler errors create which teaching moments
4. Any concepts that need particular care or that should be deferred to a later chapter

Get the author's sign-off before moving to phase 2.

### Phase 2: Teaching

Take the author through the journey step by step. The author types every line of code and runs every command. The AI teaches.

The rhythm:
1. AI explains what to do next and why — gives the exact code or command
2. Author runs it and confirms it worked (or reports if something unexpected happened)
3. AI explains what the output means
4. Repeat

The author does not need to paste routine output. A "yes", "done", or "it works" is enough. The author should share output when it's unexpected, surprising, or when a specific compiler error is a key teaching moment.

Do NOT run any commands or write any files yourself during this phase. Do NOT ask the author to commit — when a green point is reached, say "we'd commit here" and move on.

The teaching phase may surface new insights: a concept that needs more explanation, a compiler error that's more interesting than expected, a Rust quirk worth noting. Take note of these — they inform the writing phase.

### Phase 3: Writing

Once the teaching phase is complete, write the chapter prose. This is the only phase where the AI writes files and runs commands.

Follow strict TDD: write the test, run it (red), write the minimum implementation, run it (green), refactor if needed, run it again. Every piece of output in the prose comes from actually running the commands — never reconstructed.

Write and show the prose incrementally — one section at a time — and get the author's confirmation before moving on. Do not write the full chapter in one go.

Commit discipline during the writing phase:
- Commit at every green point before adding the next requirement
- Final commit message: `<Chapter Name>: chapter prose and code following TDD steps incrementally`
- After the chapter is done, update `AGENTS.md` and commit that too



## What goes where

- Exercise code: `<chapter-name>/src/main.rs` (or `lib.rs` for library crates)
- Chapter prose: `src/fundamentals/<chapter-name>.md`
- New chapters must be added to `src/SUMMARY.md`

## What to avoid

- Do not skip or merge the three phases
- Do not write code or run commands during the teaching phase
- Do not ask the author to commit during the teaching phase
- Do not write the full chapter prose in one go — section by section, with review
- Do not approximate or reconstruct compiler output — always use real output from the writing phase
- Do not re-explain TDD principles that are already covered in `src/principles/` — link instead
- Do not introduce more than one or two new Rust concepts per step without acknowledging the load on the reader
