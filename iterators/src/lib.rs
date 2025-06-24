#![allow(warnings)]
#[test]
fn iterator_demonstration() {
    let v1 = vec![23, 24, 25];
    
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
