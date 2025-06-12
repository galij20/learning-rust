fn main() {
    //let  v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let second: Option<&i32> = v.get(1);
    match second {
        Some(second) => println!("The second element is {second}"),
        None => println!("There is no second element."),
    }

    //iteration over the values
    
    for i in &v {
        println!("{i}");
    }
   
    println!("After adding 2 to each element: ");
    for i in &mut v {
        *i += 2;
        println!("{i}");
    }
    
}

