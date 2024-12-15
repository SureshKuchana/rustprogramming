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

### Memory Safety and Zero-Cost Abstractions

Rust is a system programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. It is graced with the feature of “memory safety without garbage collection,” an attribute that makes Rust one of a kind. “Memory safety” is ensuring that software is not causing any memory leaks or dangling pointers while accessing the system’s memory. In Rust, memory safety is accomplished through a system called ownership, with a set of rules that the compiler checks at compile time. This ownership system eliminates the need for garbage collection or manual memory management, thus ensuring swift execution of software and a safer memory environment. Rust’s memory management features even support concurrent programming, providing options for shared and mutable state access that ensure thread safety while reducing the risk of thread unsafety.

Zero-cost abstraction is another key concept Rust implements. In general, abstractions in programming languages allow code to be written at a high level (like in Python), while being able to run at a low level (like in C). However, these abstractions often come with a runtime cost. In contrast, Rust aims to provide many useful abstractions, such as iterators and closures, that don’t sacrifice runtime performance. This means you can write high-level code in Rust, and the Rust compiler will optimize it to run as fast as manually written low-level code.

## Why not Rust?

Big Language - lots to learn
Smaller ecosystem than C/C++
Slower iteration cycle than most languages

* Strict complier
* Slow complie times for full builds
* tests can take a while to build

## What can I build with Rust?

* Web servers
* Command-Line Interfaces
* Native Desktop Application
* In-Browser apps via Web-Assembly [makepad.dev](https://makepad.dev)
* Performance intensive libraries
* Operating Systems

## Language features

* Performance
* Strong, static, expressive type system
* Great error messages
* Modern generics
* Memory safety
* Fearless concurrency
* Cross platform
* C interoperability

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

it will generate the target folder & Cargo.lock

### Crates
The primary unit of code organization in Rust is called a crate. Your code exists as
a crate which can be distributed to the community via crates.io.

Crates in Rust are analogous to gems in Ruby or packages in JavaScript. The registry at crates.io is similar to rubygems.org or npmjs.com as the de facto community repository for
distributing and sharing code.

Making our crate a library

Cargo assumes the entry point for defining a library crate is a file src/lib.rs. Let’s
convert our current binary crate into a binary and library crate. First, we create our
library entry point:

```rs
// src/lib.rs
pub fn say_hello() {
    println!("Hello, world!");
}
```

There are two differences to this code from what was in main.rs. First, we changed
the name of the function from main to say_hello. This change is more cosmetic than
anything (in fact leaving it named main works just fine, main is only special in some
contexts).

The second change is the keyword pub before fn. This is a privacy identifier which
specifies that this function should be publicly accessible to user’s of our crate.
Without the keyword, we could call this function inside of our lib.rs file, but user’s
of our crate would not be able to call it. Note that our executable sees the library
crate the exact same as someone who included our library as a dependency in their
Cargo.toml file. This ensures a proper separation of concerns between code meant
to be executed as a binary and the actual functionality of your project.

## Macro

A macro in Rust is a piece of code that generates another piece of code

## Datatypes

```rust
// boolean 
let b: bool = true;

// unsigned integers
let i1: u8 = 1;
let i2: u16 = 1;
let i3: u32 = 1;
let i4: u64 = 1;
let i5: u128 = 1;


// unsigned integers
let u1: i8 = 1;
let u2: i16 = 1;
let u3: i32 = 1;
let u4: i64 = 1;
let u5: i128 = 1;

// floating point numbers
let f1: f32 = 1.0;
let f2: f64 = 1.0;

// platform specific integers
let p1: usize = 1;
let p2: isize = 1;

// characters, &str, String
let c: char = 'c';
let s: &str = "hello";
let s1: String = String::from("hello");

// Above data types are the scalar datatypes, bz it stores only single value

// Compound data types which stores multiple values

// arrays
let a1: [i32; 5] = [1, 2, 3, 4, 5]
let ia1 = a1[4]

// tuples
let t1: (i32, i32, i32) = (1, 2, 3);
let t2: (i32, f64, &str) = (5, 5.0, "5");

```
