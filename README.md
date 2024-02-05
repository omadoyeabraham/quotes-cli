# CLI Program for Displaying Amazing Quotes

This Rust CLI program retrieves inspirational quotes from [ZenQuotes API](https://zenquotes.io/api/quotes/) and displays them on the command line. The user can specify the number of quotes to be displayed.

## Usage

```bash
./quotescli <NUMBER OF QUOTES>
```

- `<NUMBER>`: The number of quotes to be displayed. Must be a positive integer, and ha a maximum value of 10.

## Example

```bash
./quotescli 5
```

This command will display 5 inspirational quotes on the command line.

## Dependencies

- [anyhow](https://crates.io/crates/anyhow): A library for flexible error handling in Rust.
- [clap](https://crates.io/crates/clap): A command-line argument parser for Rust.
- [cli_table](https://crates.io/crates/cli_table): A library for creating command-line tables in Rust.
- [serde](https://crates.io/crates/serde): A framework for serializing and deserializing Rust data structures.
- [reqwest](https://crates.io/crates/reqwest): An HTTP client for Rust.

## Build and Run

To build and run the program, make sure you have Rust installed. Then, run the following commands:

```bash
cargo build --release
./target/release/quotescli 3
```

This will build the program and execute it with the specified number of quotes.

Feel free to explore and customize the code according to your needs!
