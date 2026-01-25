# fsh (fowl shell)

This is a learning project with the goal of building a small shell in Rust (based on the [codecrafters challenge](https://app.codecrafters.io/courses/shell/overview)).

## Features

- Basic REPL (Read-Eval-Print-Loop) with command execution (built-in and path), argument handling (separation by whitespace only)
- Supported built-in commands:
  - `echo`
  - `exit`
  - `type`

## Roadmap

- Additional built-in commands like `pwd`, `cd`, and `history`
- Quoting & escaping (single & double quotes, backslash escaping)
- I/O redirection (stdout, stderr)
- Pipelines (built-in and multi-command pipelines)
- Autocompletion (built-in, external, path)
- History (list, execute from history, arrow navigation) & history persistence

## Installation

You can either download a binary from the [releases page](https://github.com/fowl-ow/fsh/releases) or build it from source (requires cargo / rust):

```shell
git clone https://github.com/fowl-ow/fsh.git
cd fsh
cargo install --path .
```

## Development Setup

Development requires [Rust](https://rust-lang.org/).

Add git hooks:

```shell
git config core.hooksPath .githooks
```

## Project Links

- Changelog: [CHANGELOG.md](./CHANGELOG.md)
