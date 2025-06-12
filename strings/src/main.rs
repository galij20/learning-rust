#[allow(unused_variables)]
fn main() {
 let s = String::new();

 let s = "It's LSPPDay12.".to_string();
 println!("{s}");

 let s = String::from("Hello, It's day 12 of learning Rust.");      let s = String::from("नमस्ते");
 println!("{s}");
 
 let mut s = String::from("foo");
 s.push_str(" fighters");

 let s1 = String::from("rock");
 let s2 = String::from("paper");
 let s3 = String::from("scissors");

 let s = format!("{s1}, {s2}, {s3}.");
 println!("{s}");
}

