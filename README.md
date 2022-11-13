# rustprogramming

## What is Rust?

A Language empowering everyone to build reliable & efficient software;
Rust either complies machine code or web assembly [Rust](https://www.rust-lang.org/tools/install)

## Why Rust?

1. Speed
2. Performance
3. Going Real Fast

Automatic Memory Management
A Package Manager & code formatter
More complie help with concurrency

Rust borrows a lot of ideas from other languages & is truly multi-paradigm, meaning you can write mostly functional
code or imperative code

## Why not Rust?

Big Language - lots to learn
Smaller ecosystem than C/C++
Slower iteration cycle than most languages

* Strict complier
* Slow complie times for full builds
* tests can take a while to build

## What can I build with Rust?

Web servers
Command-Line Interfaces
Native Desktop Application
In-Browser apps via Web-Assembly [makepad.dev](https://makepad.dev)
Performance intensive libraries
Operating Systems

## Language features

Performance
Strong, static, expressive type system
Great error messages
Modern generics
Memory safety
Fearless concurrency
Cross platform
C interoperability

## Getting your environment setup

* [Rustup](https://rustup.rs/) is an installer for
the systems programming language Rust
* [Cargo](https://doc.rust-lang.org/cargo/) is the Rust package manager. Cargo downloads your Rust package's dependencies, compiles your packages, makes distributable packages, and uploads them to crates.io, the Rust community’s package registry.

### Why Cargo Exists?

In Rust, as you may know, a library or executable program is called a crate. Crates are compiled using the Rust compiler, rustc. When starting with Rust, the first source code most people encounter is that of the venerable “hello world” program, which they compile by invoking rustc directly:

```bash
$ rustc hello.rs
$ ./hello
Hello, world!
```

### IDEs, RLS, Editors

* [Rust Language Server](https://github.com/rust-lang/rls)
* [Racer](https://github.com/racer-rust/racer)
* [Langserver](https://langserver.org/)
* [Clippy](https://github.com/rust-lang/rust-clippy) to add clippy

```bash
# rustup --version
# cargo --version
rustup component add clippy
# then 
cargo clippy
```

* [Rustfmt](https://github.com/rust-lang/rustfmt)

## Generating new Project

```bash
cargo new numbers
# which contains
├── Cargo.toml
└── src
└── main.rs
# or
# Running cargo new project_name by default is equivalent to 
cargo new project_name --bin
# which generates a binary project.

# Alternatively, we could have run
cargo new project_name --lib
# generate a library project
```

### Binary vs. library

A binary project is one which compiles into an executable file. For binary projects,
you can execute cargo run at the root of your application to compile and run the
executable.

A library project is one which compiles into an artifact which is shareable and can
be used as a dependency in other projects. Running cargo run in a library project
will produce an error as cargo cannot figure out what executable you want it to run
(because one does not exist). Instead, you would run cargo build to build the library.

### main.rs

For a binary project, the entry point is assumed to be located at src/main.rs .
Furthermore, inside that file, the Rust compiler looks for a function named main
which will be executed when the binary is run. Cargo has generated a main.rs file
which contains a simple “Hello, world!” application:

```rust
fn main() {
    println!("Hello, world!");
}
```

### Caro.toml

The Cargo.toml file is the manifest file for the project which uses the TOML
format. This is the entry point for describing your project as well as specifying
dependencies and configuration. The initial generated file contains the bare essentials for describing your project:

```rust
[package]
name = "numbers"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

```

### run the application

```bash
cargo run
```

it will generate the Cargo.lock
