# djsdocs

A command line utility to access the discord.js documentation.

## Installation

1. Install [Rust](https://rust-lang.org), which is required to build it
2. Install the executable with `cargo install --git https://github.com/vaporox/djsdocs`

## Usage

```
djsdocs [query] [--src=source] [--tag=tag]
```

| Parameter | Default      | Description                                                               |
|:---------:|:------------:|---------------------------------------------------------------------------|
| query     | /            | The search query. If empty, all elements get listed                       |
| src       | `discord.js` | One of `builders`, `collection`, `discord.js`, `proxy`, `rest` or `voice` |
| tag       | `main`       | One of `main`, `stable` or a specific version tag                         |
