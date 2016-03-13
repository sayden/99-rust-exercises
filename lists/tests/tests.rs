#![feature(slice_patterns)]

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
    let mut acc = vec![];
    lists::compress(vec![1, 1, 2, 2, 2, 3, 4], &mut acc);
    assert_eq!(acc, vec![1, 2, 3, 4]);

    let mut acc = vec![];
    lists::compress(vec!['a', 'a', 'b', 'b', 'b', 'c', 'd'], &mut acc);
    assert_eq!(acc, vec!['a', 'b', 'c', 'd']);
}

#[test]
fn test_pack() {
    // 9 Pack consecutive duplicates of list elements into sublists
    let data: Vec<u32> = vec![1, 1, 2, 2, 2, 3, 3, 4, 5, 6, 6, 6, 7];
    let res = lists::pack(data);
    assert_eq!(res,
               vec![vec![1, 1],
                    vec![2, 2, 2],
                    vec![3, 3],
                    vec![4],
                    vec![5],
                    vec![6, 6, 6],
                    vec![7]]);
}

#[test]
fn test_encode() {
    // 10 Run-length encoding of a list.
    let res = lists::encode(vec![1, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 1, 1, 1, 5, 5, 6]);
    assert_eq!(res,
               vec![(3, 1), (2, 2), (4, 3), (2, 4), (3, 1), (2, 5), (1, 6)]);
}

#[test]
fn test_encode_modified() {
    // 10 Run-length encoding of a list.
    let res = lists::encode_modified(vec![1, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 1, 1, 1, 5, 5, 6]);
    assert_eq!(res,
               vec![lists::EncodeType::Multiple(3, 1),
                    lists::EncodeType::Multiple(2, 2),
                    lists::EncodeType::Multiple(4, 3),
                    lists::EncodeType::Multiple(2, 4),
                    lists::EncodeType::Multiple(3, 1),
                    lists::EncodeType::Multiple(2, 5),
                    lists::EncodeType::Single(6)]);
}

#[test]
fn test_decode_modified() {
    // 12 Decode a run-length encoded list.
    let data = vec![lists::EncodeType::Single(1),
                    lists::EncodeType::Single(3),
                    lists::EncodeType::Single(1),
                    lists::EncodeType::Single(2),
                    lists::EncodeType::Multiple(3, 4),
                    lists::EncodeType::Multiple(5, 5)];
    let res = lists::decode_modified(data);
    assert_eq!(res, vec![1, 3, 1, 2, 4, 4, 4, 5, 5, 5, 5, 5]);
}

#[test]
fn test_encode_direct() {
    // 13 Run-length encoding of a list (direct solution).
    let res = lists::encode_direct(vec![1, 1, 1, 2, 2, 3, 3, 3, 3, 4, 4, 1, 1, 1, 5, 5, 6]);
    assert_eq!(res,
               vec![(3, 1), (2, 2), (4, 3), (2, 4), (3, 1), (2, 5), (1, 6)]);

}

#[test]
fn test_dupli() {
    // 14 Duplicate the elements of a list.
    assert_eq!(vec![1, 1, 2, 2, 3, 3], lists::dupli(vec![1, 2, 3]));
    assert_eq!(vec!['a', 'a', 'b', 'b', 'c', 'c'],
               lists::dupli(vec!['a', 'b', 'c']));
}


#[test]
fn test_repli() {
    // 15 Replicate the elements of a list a given number of times.
    assert_eq!(vec![1, 1, 1, 2, 2, 2, 3, 3, 3],
               lists::repli(vec![1, 2, 3], 3));
    assert_eq!(vec!['a', 'a', 'a', 'b', 'b', 'b', 'c', 'c', 'c'],
               lists::repli(vec!['a', 'b', 'c'], 3));
}

#[test]
fn test_drop() {
    // 16 Drop every N'th element from a list.
    assert_eq!(lists::drop(vec![1, 2, 3, 4, 5, 6, 7, 8], 2), [1, 3, 5, 7]);;
    assert_eq!(lists::drop(vec![1, 2, 3, 4, 5, 6, 7, 8], 3),
               [1, 2, 4, 5, 7, 8]);
    assert_eq!(lists::drop(vec![1, 2, 3], 1), vec![]);
    assert_eq!(lists::drop(vec![1, 2, 3], 5), vec![1, 2, 3]);
}
