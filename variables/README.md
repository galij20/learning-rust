#LSPPDay4
# Variables and Mutability
Variables in Rust are immutable by default. This helps us to write code in a way that takes advantage of the safety and easy concurrency that Rust offers. 
Making a variable immutable means a value bound to a variable cannot be changed. 

### Running the code
After cloning the repo, cd into the variables directory and run 
```bash
$ cargo run
```
or 
```bash
$ ./src/main.rs
```
This will execute the binaries.

### Programming steps and logic 
First, we set up a new project naming it `variables`. 
```bash
$ cargo new variables
$ cd variables
```
Then, we moved onto coding in src/main.rs file to implement various concepts in variables such as mutability and shadowing. 
```rust
fn main(){
let x = 5;
println!("The day of learning is {x}.");
x = 4;
println!("The day of learning is {x}.");
}
```
We expect this code to output 
```bash
The day of learning is 5.
The day of learning is 4.
```
but, we get an error instead.
```bash
 Compiling variables v0.1.0 (/home/bucho/git/learning-rust/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("The day of learning is {x}.");
4 |     x = 4;
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` (bin "variables") due to 1 previous
 error
```
This error is the result of the variable x or any variable in Rust that is initialized is immutable by nature. Immutability is required for variables that we know won't change values and helps track us such variables. 
Mutability is also useful though.
Adding `mut` while initializing a variable indicates that the value bound to the variable might change in the future.
Running the program after adding the keyword successfully completes and outputs the expected output.

Constants in Rust
Constants, unlike variables, cannot be made mutable by adding `mut` in it's declaration. They are declared using the keyword `const` instead of `let`.

An example of constant declaration is
```rust
const LEARNING_OR_NOT: bool = true;
```
This snippet of code declares a constant LEARNING_OR_NOT and binds the value true to it.

Shadowing in Rust
We can declare a new variable with the same name as a previous variable in Rust. The second variable is said to shadow the first variable. After this happens, any mention of the variable in later part of the code will use the value provided in the second declaration.
We can shadow a variable by using `let` with the same variable name.
An example can be seen below: 
```rust
	let x = 5;
    println!("Today is not day {x} of learning");
```
This initializes a new variable x which shadows the previous x and prints the desired output.
Compared to making a variable mutable, shadowing helps in creating variables of different data types.
