fn main() {
    let  s = "hello";

    println!("{s}, world!");

   /* let s = s + ", world!"; 
    println!("{s}"); */ 
    // Here, s is of the type &str and it is a string literal and hardcoded so cannot be changed 
    // without the using of .to_owned() methods. So, we will introduce another string type String 
    
    let mut s1 = String::from("hello");
    s1.push_str(", It's LSPPDay7"); // appended a literal to string
    println!("{s1}");
    
    let s2 = s1.clone();
    println!("s1 = {s1}");
    println!("s2 = {s2}");

    let x = 7;
    let y = x;
    println!("x = {} and y = {}", x, y);
    //Copy trait is a trait in Rust that lets you copy the value of one variable to the other.
    //It is not possible in strings whose data is stored in the heap.

    //Ownership and functions
    //Just as assignment, passing a value to a function moves or copies it.
    
    //What if we want to let a function use a value but not take ownership? 
    //Although, it can be done by passing and returning the same value, Rust has references.
   
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    change(&mut s1);

    let r1 = &s1;
    let r2 = &s1;
    println!("{r1} and {r2}");
    
    let r3 = &mut s1;
    r3.push_str(", It is LSPPDay7");
    println!("{r3}");
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String){
    some_string.push_str(", world");
    println!("The string after mutation is {some_string}"); 
} 
