extern crate lists;

#[test]
fn test_last_element() {
    // 1 Find the last element of a list.
    assert_eq!(lists::my_last(vec![18, 5, 3, 54]), 54);
}

#[test]
fn test_last_but_one() {
    // 2 Find the last but one element of a list.
    assert_eq!(lists::my_but_last(vec![18, 5, 3, 54]), 3);
}


#[test]
fn test_k_element() {
    // 3 Find the K'th element of a list. The first element in the list is number 1
    assert_eq!(lists::element_at(vec![18, 5, 3, 54], 2), 5);
}


#[test]
fn test_number_of_elements() {
    // 4 Find the number of elements of a list.
    assert_eq!(lists::my_length(vec![0, 1, 2, 3, 4, 5]), 6);
}

#[test]
fn test_reverse_list() {
    // 5 Reverse a list
    assert_eq!(lists::my_reverse(vec![0, 1, 2, 3]), [3, 2, 1, 0]);
}

#[test]
fn test_palindrome() {
    // 6 Find out whether a list is a palindrome. A palindrome can be read forward or backward; e.g. (x a m a x).
    assert_eq!(lists::is_palindrome(vec![1, 2, 3]), false);
    assert_eq!(lists::is_palindrome(vec![3, 2, 3]), true);
    assert_eq!(lists::is_palindrome(vec![3, 2, 2, 4, 2, 2, 3]), true);
}

#[test]
fn test_flatten() {
    // 7 Flatten a nested list structure.
    let mut acc1: Vec<u32> = Vec::new();
    lists::my_flatten(vec![lists::NestedList::Elem(1)], &mut acc1);
    assert_eq!(acc1, vec![1]);

    acc1 = Vec::new();
    lists::my_flatten(vec![lists::NestedList::List(vec![
                    lists::NestedList::Elem(1),
                    lists::NestedList::Elem(2),
                    lists::NestedList::Elem(3),
                    lists::NestedList::List(
                        vec![lists::NestedList::Elem(4)]
                    )
                ]),
                           lists::NestedList::List(vec![lists::NestedList::Elem(5)]),
                           lists::NestedList::Elem(6)],
                      &mut acc1);
    assert_eq!(acc1, vec![1, 2, 3, 4, 5, 6]);

    let mut acc2: Vec<char> = Vec::new();
    lists::my_flatten(vec![lists::NestedList::List(vec![
                    lists::NestedList::Elem('a'),
                    lists::NestedList::Elem('b'),
                    lists::NestedList::Elem('c'),
                    lists::NestedList::List(
                        vec![lists::NestedList::Elem('d')]
                    )
                ]),
                           lists::NestedList::List(vec![lists::NestedList::Elem('e')]),
                           lists::NestedList::Elem('f')],
                      &mut acc2);
    assert_eq!(acc2, vec!['a', 'b', 'c', 'd', 'e', 'f']);
}

#[test]
fn test_compress() {
    // 8 Eliminate consecutive duplicates of list elements.
    // let res = lists::compress(vec![1, 1, 2, 2, 2, 3, 4]);
    // assert_eq!(res, vec![1, 2, 3, 4]);

}
