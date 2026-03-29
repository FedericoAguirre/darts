# Darts

## Summary

Darts is a dart game score calculator written in [**Rust**](https://www.rust-lang.org/) language.

Darts resolve this [Python exercise](https://exercism.org/tracks/python/exercises/darts), but using Rust.

This project pretends to show how to get started in Rust by creating a basic CLI application.

## Objectives

- Show some of the basic concepts of [**Rust**](https://www.rust-lang.org/).
- Show how to initialize a Rust project using the **cargo** tool.
- Make use of **crates** (e.g., `clap`, `csv`, `serde`).
- Show unit testing and integration testing usage.
- Process multiple coordinates from CSV files.
- Compilation and deployment in Rust.

## Rust installation

You can install Rust in Linux or Windows, using the instructions in the next [link](https://www.rust-lang.org/tools/install).

It is recommended to install **rustup** program to automatically update Rust.

Install in Mac using [Homebrew](https://brew.sh/):

```shell
brew install rustup
```

Install Rust tools for [Visual Studio Code](https://code.visualstudio.com/docs/languages/rust).

## Project creation

We will use [**cargo**](https://doc.rust-lang.org/cargo/) Rust package manager to create the CLI application.

- Create a folder **darts**.

```shell
mkdir darts
```

- Change to **darts** folder.

```shell
cd darts
```

- Create the CLI appication using cargo.

```shell
cargo init --bin --name darts --vcs git
```

### cargo init

The **cargo init** command creates a new Rust project (application or library).

The **--bin** option indicates to create a **binary executable file**.

The **--name** option indicates the name of the application: **darts**.

The **--vcs** option indicates cargo to create a version control system for the application: **git**, for this case.

You can check other options, using the command:

```shell
cargo init --help
```

## Rust project folder structure

### Cargo.toml file

The **Cargo.toml** file serves to define the metadata of the executable and also serves to define the libraries used in it.

### src folder

The **src** folder contains all the modules and/or source files used in the binary.

In this case, it contains the **main.rs** file (binary entry point) and **lib.rs** file (library with score calculation logic).

### main.rs file

The **main.rs** file has the code that will be executed.

It handles command-line argument parsing and delegates to the library functions.

### lib.rs file

The **lib.rs** file contains the core library functions, including score calculation and CSV processing.

It also has the unit tests for the library.

### cargo add command

The `cargo add` command aggregates **crates** (libraries) to the current project.

When used, it adds the crate into the **Cargo.toml** file in the **dependencies** section.

The available Rust crates can be found in [crates.io](https://crates.io/).

The darts project uses the following crates:

- [**clap**](https://crates.io/crates/clap). A command-line parser.
- [**csv**](https://crates.io/crates/csv). CSV file reading and writing.
- [**serde**](https://crates.io/crates/serde). Serialization and deserialization framework (used by csv).

To add the crate to the project:

```shell
cargo add clap@4.4.11
```

**Note**: The crates can be added without specifying the version, cargo will use the latest version. However, it is a best practice to determine the version to be used.

## CSV File Processing

In addition to single coordinate mode, the darts calculator now supports processing multiple coordinates from a CSV file.

### Input CSV Format

The input CSV file must have a header row with columns named `x` and `y`. Coordinates can be integers or floating-point numbers (e.g., `3` or `3.0`).

Example `input.csv`:

```csv
x,y
0.0,0.0
1.4,1.4
4.0,4.0
11.0,11.0
invalid,0.0
0.0,NaN
```

### Output CSV Format

The output CSV file includes the original `x` and `y` columns plus a `score` column. Valid coordinates produce numeric scores (0, 1, 5, or 10). Invalid coordinates (non‑numeric, NaN, or infinite) produce `ERROR`.

Example `output.csv`:

```csv
x,y,score
0.0,0.0,10
1.4,1.4,5
4.0,4.0,1
11.0,11.0,0
invalid,0.0,ERROR
0.0,NaN,ERROR
```

### Command‑Line Options

| Option | Short | Description |
|--------|-------|-------------|
| `--csv <FILE>` | `-c` | Path to input CSV file. |
| `--output <FILE>` | `-o` | Path to output CSV file (optional). |
| `--x_coord <X>` | `-x` | Single coordinate X (mutually exclusive with `--csv`). |
| `--y_coord <Y>` | `-y` | Single coordinate Y (mutually exclusive with `--csv`). |

### Examples

Process a CSV file with explicit output path:

```shell
darts --csv coordinates.csv --output scores.csv
```

Process a CSV file with default output name (`coordinates.score.csv`):

```shell
darts --csv coordinates.csv
```

Single coordinate mode (unchanged):

```shell
darts --x_coord 0.0 --y_coord 0.0
```

### Error Handling

- If a coordinate cannot be parsed as a number, the score is set to `ERROR`.
- If a coordinate is `NaN` or infinite, the score is set to `ERROR`.
- Missing `x` or `y` columns in the input CSV cause the program to fail.
- The output file always includes a header row, even if the input has no data rows.
