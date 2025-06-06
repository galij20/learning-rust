fn main() {
    let number = 3;
    //simple if-else expressions
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = 6;
    //else if ladder in Rust
    //there are no parantheses used in condition expressions
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    //using if in a let statement
    let condition = true;
    let number = if condition {6} else {5};
    println!("Today is LSPPDay{number}.");

    //loops and repetition
    //returning using loop keyword 
    let mut counter = 0;
    let day = loop{
        counter += 1;
        if counter == 6{
            break counter;
        }
    };
    println!("Today is day {day} of LSPP.");
    
    let mut counter  = 0;
    'counting: loop{
        println!("{counter} ");
        counter += 1;
        if counter == 6{
            break 'counting ;
        }
    }
    //while loop
    while counter > 0 {
        println!{"Don't stop!!"};
        counter -= 1;
    }
    //for loop
    for number in 1..6{
        println!("I have completed day {number} in learning Rust.");
    }

}
