![Group 1 (2)](https://github.com/Hollowloki/Cecile/assets/104183502/ebdd205b-d226-4597-a033-39c6bd4fd8a9)

<h1 align="center">The Cecile Programming Language</h1>

<p align="center">
  <img src="https://img.shields.io/crates/d/Cecile?style=for-the-badge" />
  <img src="https://img.shields.io/crates/l/Cecile?style=for-the-badge" />
  <img src="https://img.shields.io/github/stars/Hollowloki/Cecile?style=for-the-badge&logo=trustpilot" />

</p>

## ⭐ Introduction

The Cecile, a little programming language designed for simplicity and flexibility.

## 🍎 Overview

Welcome to the Cecile programming language.

This language is part of my journey into learning compiler and interpreted languages. It's inspired by [ajeetdsouza/loxcraft](https://github.com/ajeetdsouza/loxcraft), a great Rust project that implemented the Lox language.

The syntax of Cecile is influenced by traditional programming languages like JavaScript, Go, and Rust, with a strong emphasis on developer experience, readability and ease-of-use.

Cecile is written in Rust. Some of performance matter part is written in Unsafe Rust that makes fast enough to compete with traditional interpreted languages like Javascript, Python.

##

Important note: **This project is built for learning purposes**. The code quality of this project is definitely not production ready.

## 📕 Features

#### Language features

- [x] Interpreted
- [x] Bytecode compiler
- [x] Garbage collected in runtime
- [x] Type Supported
- [x] Basic types, Array
- [x] Control flow statements
- [x] Object Oriented Programming
- [x] Stack tracing
- [x] Repl

#### Ongoing features

- [ ] Online playground

---

#### For compiled version of Cecile

- [ ] Make compiled
- [ ] Self-hosted
- [ ] Standart library
- [ ] Module system

## 🔥 Syntax examples

#### Variable Declaration

```rust
// Variable Declaration
let number: int = 1;
let string: String = "string";

// Ofcource you don't need to write type everytime you declare variable
let number = 2;
let string = "hello cecile";

```

#### Function Declaration

```rust
fn say_hello() -> String {
  return "hello";
}

println say_hello() // Out: "hello"
```

#### Type Declaration

```rust
type Point {
  x: int,
  y: int,
}

impl Point {
  fn new(x: int, y: int) {
    self.x = x;
    self.y = y;
  }
}

let point: Point = Point();
```

#### Array & Builtin function

```rust
let arr = [1, 2, 3, 4];
arr.push(5);

println arr; // Out: [1, 2, 3, 4, 5]
```

## ⚙️️ Build Guide

### 🦀 Install Rust

We recommend installing Rust using [rustup](https://www.rustup.rs/). You can install `rustup` as follows:

- macOS or Linux:

  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Windows (64-bit):

  Download the [Windows 64-bit executable](https://win.rustup.rs/x86_64) and follow the on-screen instructions.

### 🐙 Install Cecile

```bash
# Install 'Cecile'
$ cargo install Cecile --locked
```

Now to use Cecile language, in your terminal, run:

```bash
Cecile
```
