fn main(){
    //mutability
    let mut x = 5;
    println!("The day of learning is {x}.");
    x = 4;
    println!("The day of learning is {x}.");

    //constants
    const LEARNING_OR_NOT: bool = true;
    println!("Am I currently learning or not?");
    println!("{LEARNING_OR_NOT}");
}
