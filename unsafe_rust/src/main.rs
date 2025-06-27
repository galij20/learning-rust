fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    unsafe{
        dangerous();
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe fn dangerous() {
    println!("This is Day 26 of Learning Rust.");
}

