#![allow(warnings)]
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); 
    println!("{v1_iter:?}");

    for val in v1_iter {
        println!("Got: {val}");
    }
    let v2 = vec![23, 24, 25];

    let v2_iter = v1.iter();
    let total: i32 = v2_iter.sum();
   
    println!("The sum of the elements in the vectr using iterator is {total}");

    let v3: Vec<i32> = vec![1, 2, 3];

    let v4: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v4, vec![2, 3, 4]);
}


