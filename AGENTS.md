# Learn Rust with Tests — Project Context

## Goal

Build "Learn Rust with Tests" — an mdBook teaching Rust via TDD, inspired by Learn Go with Tests (LGWT). Published at https://quii.github.io/learn-rust-with-claude-and-tests/

## Constraints & Preferences

- Author writes/runs all code themselves (honesty/integrity of the book)
- AI is teacher/collaborator, not ghostwriter — disclosed in README
- Follow LGWT rhythm: brisk, trusts the reader, teaching voice not documentation voice
- Compiler errors are teaching moments, not things to skip past
- Real terminal output only — never reconstructed; strip machine-specific paths (e.g. `/Users/quii/...` → `/path/to/<chapter-name>`)
- Commit at every green point before adding next requirement
- `target/`, `book/`, and `.idea/` are gitignored

## Collaboration model

Plan chapter together → AI executes (TDD incrementally, real output) → author reviews and edits. The write-chapter skill at `.opencode/skills/write-chapter/SKILL.md` governs the detailed authoring process.

## Repo

- GitHub: `git@github.com:quii/learn-rust-with-claude-and-tests.git`
- Published via GitHub Actions on push to `main` → GitHub Pages

## Current state of the code

### structs/src/lib.rs

Final state:

```rust
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perimeter_of_rectangle() {
        let r = Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(r.perimeter(), 30.0);
    }

    #[test]
    fn area_of_rectangle() {
        let r = Rectangle { width: 10.0, height: 5.0 };
        assert_eq!(r.area(), 50.0);
    }
}
```

## Completed chapters

1. **Hello World** (`src/fundamentals/hello-world.md`, code in `hello-world/`)
   - Covers: `cargo new`, `fn`, `&str`/`String`, `format!`/`println!`, `if` as expression, variable shadowing, `Option<&str>`, `unwrap_or`
   - Testing: `#[cfg(test)]`, `mod tests`, `#[test]`, `assert_eq!`

2. **Integers** (`src/fundamentals/integers.md`, code in `integers/`)
   - Covers: `cargo new --lib`, `i32`, integer type family, no silent coercion, `///` doc comments, `cargo doc`, doc-tests, overflow behaviour
   - Testing: doc-tests as first-class tests alongside unit tests

3. **Strings and Iteration** (`src/fundamentals/strings-and-iteration.md`, code in `strings-and-iteration/`)
   - Covers: `String` vs `&str`, `let mut`, `String::new()`, `push_str`, `for _ in 0..n`, `usize`, refactoring to `s.repeat(n)`
   - Testing: tests drive you to the right types

4. **Structs** (`src/fundamentals/structs.md`, code in `structs/`)
   - Covers: `struct`, `pub` on fields, `f64`, field access with `.`, `impl`, `&self`, ownership contrast (`r: Rectangle` takes ownership, `&self` borrows)
   - Testing: writing a test before the type exists; fixing compiler errors one at a time

## Key files

- `book.toml` — mdBook config, playground enabled
- `src/SUMMARY.md` — table of contents
- `src/introduction.md` — book landing page
- `src/fundamentals/hello-world.md` — first chapter
- `src/principles/` — 6 principle docs
- `.opencode/skills/write-chapter/SKILL.md` — chapter authoring process
- `README.md` — AI transparency disclosure, local setup instructions
- `.github/workflows/deploy.yml` — GitHub Actions → GitHub Pages
