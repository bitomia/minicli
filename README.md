# minicli

minicli is a minimal command line interface crate written in Rust. It provides a simple way to handle multiple subcommands and options.

## Features

- Simple command parsing with support for commands, subcommands, and flags.
- Zero dependency

## Example

Run the example application with:

```bash
cargo run -- <command> [options]
```

Where `<command>` can be one of:
  - `clients` for managing clients.
  - `help` for displaying help information.

```bash
cargo run -- clients ls
```

