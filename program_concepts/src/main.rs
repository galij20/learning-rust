fn main() {
    //datatypes in rust
    //scalar
    let x : i32 = -4;
    println!("The value of x is {x}");
    //float
    let x : f32 = 3.9;
    println!("The value of x is {x}");
    //arithmetic operations
    let sum = 5 + 20;
    let difference : f64 = 8.5 - 3.2;
    let product = 8 * 9;
    let quotient = 53.0 / 8.0;
    let remainder = 48 % 4;
    
    println!("Sum = {sum} \nDifference = {difference}");
    println!("Product = {product} \nQuotient = {quotient}");
    println!("Remainder = {remainder}");
    
    //bool type
    let t = true;
    let f: bool = false;
    println!("This is LSPP Day {t} and not LSPP day {f}.");
    //the character type 
    let c = 'z';
    let z: char = 'Z'; //expicit type annotation
    println!{"{c} and {z}"};

    //Tuple type 
    let tup:(i32, f32, u8) = (50, 41.9, 254);
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    //Array type
    let a: [u32; 5] = [1, 4, 9, 16, 25];
    let b  = a[0];
    println!("The first element of array a is {b}");

    new_function();
    sum_function(4, 3);
    let day = current_day();
    println!("The current learning day is {day}");
}
fn new_function(){
    println!("This is a value from another function");
}
fn sum_function(x: u32, y: u32){
    let sum1 = x + y;
    println!("The sum of two numbers is {sum1}");

}
fn current_day() -> u32{
    let yesterday = 4;
    yesterday + 1
}
