# Things that can be accomplished with **RUST**

- CLI apps
- web servers
- developer tools

# Introduction

Rust helps write faster and reliable software. Rust helps balance technical capacity and developer experience

## Who Rust is For

1. **Teams of Developers**: Rust a productive tool for collaborating within large teams of developers with varying levels of systems programming knowledge. In Rust, the compiler plays a gatekeeper role by refusing to compile coode with these elusive bugs, including concurrency bugs. Hence, enabling tem spend their time focusing on the program's logic rather than chasing down bugs.

Rust brings contemporary developer tools to the systems programming world.

- Cargo, dependency manager and build tool, makes adding, building, compiling and managing dependencies painless and consistent across the RUST ecosystem
- The Rustfmt formatting tool ensures a consistent coding style across developers.
- The rust-analyzer powers IDE integration for code completion and inline error messages

2. **Students**: For students interested in learning about systems concept. Using Rust, people have learned about topics like operating systems development. The Rust teams want to make systems concepts more accessible to more people, especially those new to programming

3. **Companies**: Companies use Rust in production for variety of tasks, including CLI tools, web services, DevOps tooling, embedded devices, audio and video analysis and transcoding, cryptocurrencies, bioinformatices, search engines, Internet of Things applications, machine learning and even major parts of the Firefox web browser.

4. **Open Source Developers**: for people willing to build the Rust programming language, community, developer tools and libraries.

**Note**: After installing rust, will need a linker, a program Rust uses to join its compiled outputs into one file. [Install a C compiler] which include a linker

We can open rust one page documentation from the command line using `rustup doc`

Rust files ends with the .rs extension. If a file is made up of more than a word, the convention is to use an underscore to seperate them.

## Compiling and Running Rust Program

**Step 1**: rustc main.rs
**Step 2**: ./main

## Anatomy of Rust Program

`fn main() {}`

The above line define a function named _main_. The _main_ function is special, It is the first code that runs in every executable Rust program.

**Note**: Sticking to a standard style across Rust projects, use an automatic formatter tool caled _rustfmt_ to format to format code in a particular style

`println!("Hello, world!");`

The above line prints text to the screen. Note, Rust style is to indent with four spaces and not a tab. The `println!` calls a Rust macro. if it was a function, it will be `println`. In Rust, functions and macros are different.

# Hello, Cargo!

Cargo is Rust's build system and package manager. It is a tool used to manage Rus projects, it helps in building our code, downloading libraries otherwise known as dependencies

Starting a project using cargo enables adding dependencies to project easy.

use (cargo --version) to check if cargo is installed.

## Creating a project with Cargo

run `cargo new hello_cargo` in our terminal or command line

TOML stands for `Tom Obvious, Minimal Lnaguage` which is cargo's configuration format, just like a package.json file in a JS project.

In Rust, packages of code are referred to as

## Building and running a Cargo Project

from the project directory created via cargo, run `cargo build` - this will enable us to build or compile our rust program.

Alternatively, we can compile and run our rust program using `cargo run` which is more convenient to using `cargo build`

Let say, we don't want to run or compile our program. Instead want to see if it will compile without creating an executable we can use the `cargo check` command.

## Building for Release

When a project is ready for release, we can use `cargo build --release` to compile it with optimizations. This command will create an executable in `target/release` instead of `target/debug`
