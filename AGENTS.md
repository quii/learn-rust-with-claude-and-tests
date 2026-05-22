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

### integers/src/lib.rs

Final state:

```rust
/// Adds two integers together.
///
/// # Examples
///
/// ```
/// assert_eq!(integers::add(2, 2), 4);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_integers() {
        assert_eq!(add(2, 2), 4);
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

## Next steps

1. Plan and write the next chapter — strings or iteration following LGWT order

## Key files

- `book.toml` — mdBook config, playground enabled
- `src/SUMMARY.md` — table of contents
- `src/introduction.md` — book landing page
- `src/fundamentals/hello-world.md` — first chapter
- `src/principles/` — 6 principle docs
- `.opencode/skills/write-chapter/SKILL.md` — chapter authoring process
- `README.md` — AI transparency disclosure, local setup instructions
- `.github/workflows/deploy.yml` — GitHub Actions → GitHub Pages
