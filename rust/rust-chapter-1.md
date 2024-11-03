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
