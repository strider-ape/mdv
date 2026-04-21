# mdv

A CLI tool to render markdown files in the terminal with styling.

## Features

- Headings in bold cyan
- Code blocks with dark grey background
- Inline code with styling
- Bold and italic text support

## Installation

```bash
cargo install --path .
```

Or build manually:

```bash
cargo build --release
cp target/release/mdv ~/bin/mdv  # or add to PATH
```

## Usage

```bash
mdv <markdown-file>
```

## Example

```bash
mdv README.md
```

## Dependencies

- clap
- pulldown-cmark
- crossterm