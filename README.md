# Darts

## Summary

Darts is a dart game score calculator written in [**Rust**](https://www.rust-lang.org/) language.

Darts resolves this [Python exercise](https://exercism.org/tracks/python/exercises/darts), but using Rust.

This project pretends to show how to get started in Rust by creating a basic CLI application.

## Objectives

- Show some of the basic concepts of [**Rust**](https://www.rust-lang.org/).
- Show how to initialize a Rust project using the **cargo** tool.
- Make use of **crates**.
- Show unit testing and integration testing usage.
- Compilation and deployment in Rust.

## Rust instalation

You can install Rustin Linux or Windows, using the instructions in the next [link](https://www.rust-lang.org/tools/install).

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

### src folder

### main.rs file
