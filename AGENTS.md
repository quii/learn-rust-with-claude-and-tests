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

### hello-world/src/main.rs

Final state:

```rust
fn main() {
    println!("{}", greet(None));
}

fn greet(name: Option<&str>) -> String {
    let name = name.unwrap_or("World");
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets_world_by_default() {
        assert_eq!(greet(None), "Hello, World!");
    }

    #[test]
    fn greets_a_person_by_name() {
        assert_eq!(greet(Some("Alice")), "Hello, Alice!");
    }
}
```

## Key decisions

- **mdBook over GitBook** — static site, Rust Playground integration, actively maintained
- **Code inline with chapters** — exercise code lives at `<chapter-name>/src/`, not a separate `exercises/` dir
- **Principles as book section** — `src/principles/` is published; chapters link to it rather than re-explaining TDD inline
- **Hello World chapter flow**: `greet()` → `greet(name: &str)` → `greet(name: Option<&str>)` — parameters introduced via compiler error; `Option` introduced last as the idiomatic substitute for default arguments
- **Rust has no default arguments** — idiomatic substitute is `Option<T>` with `unwrap_or`

## Completed chapters

1. **Hello World** (`src/fundamentals/hello-world.md`, code in `hello-world/`)
   - Covers: `cargo new`, `fn`, `&str`/`String`, `format!`/`println!`, `if` as expression, variable shadowing, `Option<&str>`, `unwrap_or`
   - Testing: `#[cfg(test)]`, `mod tests`, `#[test]`, `assert_eq!`

## Next steps

1. Plan and write the next chapter — integers/variables is the natural next step following LGWT order

## Key files

- `book.toml` — mdBook config, playground enabled
- `src/SUMMARY.md` — table of contents
- `src/introduction.md` — book landing page
- `src/fundamentals/hello-world.md` — first chapter
- `src/principles/` — 6 principle docs
- `.opencode/skills/write-chapter/SKILL.md` — chapter authoring process
- `README.md` — AI transparency disclosure, local setup instructions
- `.github/workflows/deploy.yml` — GitHub Actions → GitHub Pages
