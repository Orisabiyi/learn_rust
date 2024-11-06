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
