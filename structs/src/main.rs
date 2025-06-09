//structures are custom datatypes that work with multiple datatypes
//defined using the struct keyword
#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//suppose we want to create structs that define certain values
//we use tuple structs

// we use the struct by creating an instance in the following way
fn main() {
    let mut user1 = User{  // to have a mutable element, the whole struct must be made mutable
        active: true,
        username: String::from("bucho"),
        email: String::from("bucho3@proton.me"),
        sign_in_count: 1, 
    };
    
    println!("The email of the first user is {}", user1.email);
    println!("The username of the first user is {}", user1.username);
    println!("The sign_in_count of the first user is {}", user1.sign_in_count);
    println!("Is the user active? {} ", user1.active);
    
    //mutating the fields of a struct 
    user1.email = String::from("bucho3@gmail.com");
    println!("The email of the first user is {}", user1.email);
    println!("{user1:#?}");
    

    //to create a user using the function
    let mail = String::from("bucho3@protn.me");
    let name = String::from("buccho");
    let func_user = build_user(mail, name);
    
    println!("{func_user:#?}");
    //suppose we want to replicate a previous instance without using the dot operator for every
    //field
    let user2 = User{
        email: String::from("example2@proton.me"),
        ..user1 // using this syntax specifies that any remaining field will get their values 
    };
    println!("{user2:#?}");          // from the corresponding fields in user1

}

//below is a function that creates a user with email and username as arguments
//we use the field init shorthand to lessen the syntax
fn build_user(email: String, username: String) -> User {
    User{
        active: true,
        username, 
        email,
        sign_in_count: 1,
    }
}
