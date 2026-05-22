# Learn Rust with Tests

_Learn Rust by writing tests, guided by the principles of TDD._

I am the author of [Learn Go with Tests](https://quii.gitbook.io/learn-go-with-tests) - a reasonably popular way of learning Go and TDD, and I am taking the same approach for Rust. The idea is simple: learn Rust incrementally, one small test at a time. Each chapter introduces a language concept or technique and uses Test-Driven Development to explore it.

## Reading the book

The book is published at: _TODO: add URL once hosted_

To run it locally:

```sh
# Install Rust (if you haven't already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install mdBook
cargo install mdbook

# Serve the book with live reload
mdbook serve --open
```

## A note on AI

This book was written with AI assistance. Claude was used as a teacher and collaborator — explaining Rust concepts, suggesting what to write next, and reviewing the work.

Every piece of code in this book was written and run by the human author. The journey through each chapter is real: the tests were written first, watched fail, made to pass, and refactored — in that order, by hand. The book was not generated; it was written.

We mention this because we think transparency matters, and because the distinction is important to the integrity of the material. A book about learning through doing is only worth reading if the author actually did it.

## Structure

```
src/
├── fundamentals/   # Language chapters, each taught through TDD
└── principles/     # The ideas behind the approach — TDD cycle, outside-in, etc.
```

The principles section is worth reading alongside the fundamentals, not just after. Each chapter links to the relevant principles as they come up.

## Contributing

This is a work in progress. Issues and PRs welcome.
