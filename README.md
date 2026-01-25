# fsh (fowl shell)

This is a rust learning project with the initial goal of building a POSIX compliant shell.

## Features

- Command execution (built-in and path) with basic argument handling (separation by whitespace only)
- Supported built-in commands
  - `echo`
  - `exit`
  - `type`

## Project Roadmap

## Installation

You can either download a binary from the Releases page or build it from source (requires cargo / rust):

```shell
git clone https://github.com/fowl-ow/fsh.git
cd fsh
cargo install --path .
```

## Development Setup

Add git hooks

```shell
git config core.hooksPath .githooks
```

## Project Links

- Changelog: [CHANGELOG.md](./CHANGELOG.md)
