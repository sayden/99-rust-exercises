extern crate ninety_nine_library;

use ninety_nine_library as lib;

fn main() {

    // 1 Find the last element of a list.
    let res = lib::lists::my_last(vec![18, 5, 3, 54]);
    println!("my_last of (18, 5, 3, 54) is '{}'", res);

    // 2 Find the last but one element of a list.
    let res = lib::lists::my_but_last(vec![18, 5, 3, 54]);
    println!("my_but_last of (18, 5, 3, 54) is '{}'", res);

    // 3 Find the K'th element of a list. The first element in the list is number 1
    let pos = 2;
    let res = lib::lists::element_at(vec![18, 5, 3, 54], pos);
    println!("element_at {} of (18, 5, 3, 54) is '{}'", pos, res);

    // 4 Find the number of elements of a list.
    let res = lib::lists::my_length(vec![0, 1, 2, 3, 4, 5]);
    println!("my_length of (0,1,2,3,4,5) is '{}'", res);

    // 5 Reverse a list
    let res = lib::lists::my_reverse(vec![0, 1, 2, 3]);
    print!("my_reverse of (0, 1, 2, 3) is (");
    for e in &res {
        print!(" {},", e);
    }
    println!(" )");

    // 6 Find out whether a list is a palindrome. A palindrome can be read forward or backward; e.g. (x a m a x).
    let mut res = lib::lists::is_palindrome(vec![1, 2, 3]);
    println!("is (1, 2, 3) palindrome? {}", res);

    res = lib::lists::is_palindrome(vec![3, 2, 3]);
    println!("is (3, 2, 3) palindrome? {}", res);

    res = lib::lists::is_palindrome(vec![3, 2, 3]);
    println!("is (3, 2, 2, 3) palindrome? {}", res);

    // 7 Flatten a nested list structure.
    lib::lists::my_flatten(lib::lists::NestedList::NestedList(lib::lists::NestedList::Elem(2)));
    // println!("{}", res);

}
