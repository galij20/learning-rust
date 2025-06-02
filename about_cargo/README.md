#LSPPDay2
Cargo - Rust's Build system and package manager. Rustaceans use Cargo to manage Rust projects. It handles building codes to downloading and building libraries.  The simplest programs can be built with
```rust
rustc program.rs
```
but as we scale our projects more and more, we'll need to add dependencies. 
You can install cargo through the following methods:
[Rust-doc](https://doc.rust-lang.org/cargo/getting-started/installation.html)
On Linux and MacOS systems, it can be done as follows: 
```bash
curl https://sh.rustup.rs -sSf | sh
```
After having cargo as well as rust installed we can check whether Cargo is installed or not by entering the following in the terminal: 
```bash
 cargo --version
```
We can create a project using Cargo by running:
```bash
$ cargo new <project_name>
$ cd <project_name>
```
We will see that Cargo generates one file and a directory:
- A Cargo.toml file and 
- a src directory with a main.rs file inside
It is initializes a new Git repo along with a .gitignore file.

After entering the project directory, building and running binaries is easy. As we scale our project, adding dependencies will also be convenient.
Now running
```bash 
$ cargo build 
```
will compile the source code in src/ and keep the binaries at target/debug/ directory. Adding a `--release` will optimize the compiling and the build process and output the binaries at target/release/.

We can run 
```bash
$ cargo run
```
to execute the debug or the release binaries.
In practice, Rustaceans run 
```bash
$ cargo check
```
which just checks the possibility of a program running and after finishing the code, run `cargo build` or `cargo run` to check the product.

