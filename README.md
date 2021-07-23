# DJS Docs

This is a command line utility to access the discord.js documentation.

## Installation

1. Install [Rust](https://rust-lang.org), which is required to build it
2. Install the executable with `cargo install --git https://github.com/vaporox/djsdocs`

## Usage

```
djsdocs <query> [--compact] [--force] [--src=source]
```

| Parameter | Default  | Description                           |
|:---------:|:--------:|---------------------------------------|
| query     | /        | The search query, e.g. `User`         |
| compact   | off      | Whether to put lists on a single line |
| force     | off      | Whether to ignore the cache           |
| src       | `stable` | The source of the docs, e.g. `master` |
