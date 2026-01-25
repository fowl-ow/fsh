# fsh (fowl shell)

This is a rust learning project with the initial goal of building a POSIX compliant shell.

## Features

- Command execution (built-in and path) with basic argument handling (separation by whitespace only)
- Supported built-in commands
  - `echo`
  - `exit`
  - `type`

## Dev Setup

Add git hooks

```shell
git config core.hooksPath .githooks
```
