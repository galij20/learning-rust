use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct LSPPDay27;

fn main() {
    LSPPDay27::hello_macro();
}
