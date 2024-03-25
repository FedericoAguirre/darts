# Darts

## Summary

Darts is a dart game score calculator written in [**Rust**](https://www.rust-lang.org/) language.

Darts resolve this [Python exercise](https://exercism.org/tracks/python/exercises/darts), but using Rust.

This project pretends to show how to get started in Rust by creating a basic CLI application.

## Objectives

- Show some of the basic concepts of [**Rust**](https://www.rust-lang.org/).
- Show how to initialize a Rust project using the **cargo** tool.
- Make use of **crates**.
- Show unit testing and integration testing usage.
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

In this case, it only contains the **main.rs** file.

### main.rs file

The **main.rs** file has the code that will be executed.

It also has the **unit tests** in it.

### cargo add command

The `cargo add` command aggregates **crates** (libraries) to the current project.

When used, it adds the crate into the **Cargo.toml** file in the **dependencies** section.

The available Rust crates can be found in [crates.io](https://crates.io/).

The darts project uses 2 crates:

- [**clap**](https://crates.io/crates/clap). A command-line parser.
- [**num-complex**](https://crates.io/crates/num-complex). Complex numbers for Rust.

To add both crates to the project:

```shell
cargo add clap@4.4.11
cargo add num-complex@0.4.4
```

**Note**: The crates can be added without specifying the version, cargo will use the latest version. However, it is a best practice to determine the version to be used.
