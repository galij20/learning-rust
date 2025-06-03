#LSPPDay3

# Project - Programming a guessing game

A hands-on project from the book to introduce a lot of concepts used in Rust that will be later discussed in the book. A guessing game is beginner programming problem where a random integer from 1 to 100 is generated and the player is prompted to enter a guess.
New concepts such as `let`, `match`, functions, use of external crates and such more were introduced. The concepts are to be further explained in the book.

### Running the code
After cloning the repo, cd into the guessing_game directory and run 
```bash
$ cargo run
```
or 
```bash
$ ./src/main.rs
```
This will execute the binaries.

### Programming steps and logic 
Programming for the project went this way.
First, we set up a new project naming it `guessing_game`.
```bash
$ cargo new guessing_game
$ cd guessing_game
```
Then we moved onto changing the code in src/main.rs to allow the player to input a guess.
```rust
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

This code introduced the `io` input/output library to obtain user input and then print the result as output. The `io` library comes from the standard library `std` and is brought into scope by:
```rust
use std::io;
```
`std::io` provides us with a number of useful features.
This basic program prints the various statements and takes input from the user from the snippet 
```rust
io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");
```
The `let` command initializes a variable and `mut` makes the variable mutable.
`String::new()` generates a new and empty instance of a string.
The `.read_line(&mut guess)` calls the `read_line` method and by passing `&mut guess` takes whatever the user types into standard input and appends into the string `guess`. 
`.expect()` handles potential error with result.'
`read_line` also returns a `Result` value which is an enumeration whose variants are `Ok` and `Err`.
The line 
```rust
println!("You guessed: {}", guess);
```
outputs `You guessed: guess` where guess is it's value.

Then, for generating a number we used the `rand` crate by editing the Cargo.toml file as
```toml
[dependencies]
rand = "0.8.5"
```
On building the project, `rand` along with its dependencies were built.
Cargo also generated a Cargo.lock file to ensure that the build was reproducible.

Generating a new number was done by bringing `rand::Rng` into scope.
Then we compared the guess with the secret number by using `match` expression after bringing `std::cmp::Ordering` into scope.

Then, multiple guesses were allowed with the use of the `loop` keyword.
Furthermore, breaking after correct guess and handling invalid inputs were integrated to the project.
