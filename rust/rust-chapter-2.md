# Programming a Guessing Game

We will be learning about `let, match, methods`, associated functions, external crates and more

## Task

A games that generates a random integer between 1 and 100. It will then prompt player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high.

## Processing a Guess

The first step to implement a game like this, is getting user input, process the input and check to see if the input is what is expected.

## Explanation

`use std::io;` It is used to obtain user input and then print the result as output. The `io` library comes from the standard library known as `std`

**Note**: By default, Rust has a set of Items defined in the Standard Library that it brings to the scope of every program called _prelude_

### Storing values with variables

`let mut guess = String::new();` alot is happening on this line hence, let break it down.

`let apple = 5;` This line creates a new variable named apples and binds it to the value 5. In Rust, variables are immutable by default. To then make variables mutable, we add the keyword `mut` before the variable name

`let apples = 5; //immutable`
`let mut bananas = 5; //mutable`

`String::new();` it returns a new instance of a string and the method or function `::new()` is called an associated funtion it helps create a new empty string

### Receiving User Input

In Rust, the keyword `use` is for importing libraries needed in a program.

`use std::io` this is used to import the standard input and output functionality. We then call the `stdin` from `io` module which allows to handle user input.

`io::stdin().read_line(&mut guess);` after calling standard input from the `io` module. We then call the `read_line` function to help read users input, we then passed `&mut guess` as argument to tell it what string to store the user input in.

`read_line` function takes whatever user inputs exist and append it to the referenced string without overwriting it contents.

## Generating a Secret Number

Rust doesn't yet include a random number functionality in its standard library, we could use `rand crate` which will help in generating random numbers

### Using Crate

Crate is a collection of Rust source code files. The project we build is a known as _binary crate_, while the `rand` crate is a _library crate_ which contains code that is intended to be used in other programs and can't be executed on its own.

Before using the rand crate in our source code, we have to include the crate in the project Cargo.toml file which will look like this:

```
[dependencies]
rand <library name> = "0.8.5" <version number>
```
