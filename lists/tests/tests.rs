#![feature(slice_patterns)]

extern crate lists;

#[test]
fn test_last_element() {
    // 1 Find the last element of a list.
    let vec = vec![18,5,3,54];

    assert_eq!(lists::my_last_recursive(vec.as_slice()), Some(&54));
    assert_eq!(lists::my_last(vec), 54);

    let empty: Vec<i32> = Vec::new();
    assert_eq!(lists::my_last_recursive(&empty.as_slice()), None);
}

#[test]
fn test_last_but_one() {
    // 2 Find the last but one element of a list.
    let vec = vec![18, 5, 3, 54];

    assert_eq!(lists::my_but_last_recursive(vec.as_slice()), Some(&3));
    assert_eq!(lists::my_but_last(vec), 3);

    let empty: Vec<i32> = Vec::new();

    assert_eq!(lists::my_but_last_recursive(empty.as_slice()), None);
}


#[test]
fn test_k_element() {
    // 3 Find the K'th element of a list. The first element in the list is number 1
    let vec: Vec<u32> = vec![18, 5, 3, 54];
    assert_eq!(lists::element_at(vec.clone(), 2), Some(5));

    assert_eq!(lists::element_at(vec, 5), None);
}


#[test]
fn test_number_of_elements() {
    // 4 Find the number of elements of a list.
    let vec: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
    
    assert_eq!(lists::my_length(vec.clone()), 6);

    assert_eq!(lists::my_length_fold(vec.clone()), 6);
    assert_eq!(lists::my_length_fold(vec.as_slice()), 6);
}

#[test]
fn test_reverse_list() {
    // 5 Reverse a list
    assert_eq!(lists::my_reverse(&vec![0, 1, 2, 3]), [&3, &2, &1, &0]);
}

#[test]
fn test_middle_index(){
    assert_eq!(lists::middle_index(5), 2);
    assert_eq!(lists::middle_index(4), 1);
    assert_eq!(lists::middle_index(3), 1);
    assert_eq!(lists::middle_index(7), 3);
}

#[test]
fn test_palindrome() {
    // 6 Find out whether a list is a palindrome. A palindrome can be read forward or backward; e.g. (x a m a x).
    assert_eq!(lists::is_palindrome(vec![1, 2, 3]), false);
    assert_eq!(lists::is_palindrome(vec![3, 2, 3]), true);
    assert_eq!(lists::is_palindrome(vec![3, 2, 2, 4, 2, 2, 3]), true);
    assert_eq!(lists::is_palindrome(vec![3, 2, 2, 4, 2, 2, 1]), false);
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

#[test]
fn test_split() {
    // 17 Split a list into two parts; the length of the first part is given.
    assert_eq!(lists::split(vec![1, 2, 3, 4, 5], 2),
               vec![vec![1, 2], vec![3, 4, 5]]);
}

#[test]
fn test_slice() {
    // 18 Extract a slice from a list.
    assert_eq!(lists::slice(vec![1, 2, 3, 4, 5], 2, 4), vec![2, 3, 4]);
    assert_eq!(lists::slice(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i'], 3, 6),
               vec!['c', 'd', 'e', 'f']);
}

#[test]
fn test_rotate() {
    // 19 Rotate a list N places to the left.
    assert_eq!(lists::rotate(vec![1, 2, 3, 4, 5], 2), vec![3, 4, 5, 1, 2]);
    assert_eq!(lists::rotate(vec!['a', 'a', 'b', 'b', 'c', 'd', 'e', 'f', 'g'], 5),
               vec!['d', 'e', 'f', 'g', 'a', 'a', 'b', 'b', 'c']);
}

#[test]
fn test_remove() {
    // 20 Remove the K'th element from a list.
    assert_eq!(lists::remove(vec![1, 2, 3, 4], 2), vec![1, 3, 4]);
}

#[test]
fn test_insert_at() {
    // 21 Insert an element at a given position into a list.
    assert_eq!(lists::insert_at(2, vec![1, 2, 3, 4, 5], 99),
               vec![1, 99, 2, 3, 4, 5]);
}

#[test]
fn test_range() {
    // 22 Create a list containing all integers within a given range.
    assert_eq!(lists::range(3, 9), vec![3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_rnd_select() {
    // 23 Extract a given number of randomly selected elements from a list.
    let data = vec![1, 2, 3, 4, 5];
    let res: Vec<u32> = lists::rnd_select(data.clone(), 3);
    assert_eq!(res.len(), 3);
    for i in res {
        assert_eq!(data.contains(&i), true);
    }
}

#[test]
fn test_rnd_lotto() {
    // 24 Lotto: Draw N different random numbers from the set 1..M.
    let res = lists::rnd_select_lotto(5, 20);
    assert_eq!(res.len(), 5);

    for i in res {
        assert_eq!(i <= 20 && i >= 1, true);
    }
}

#[test]
fn test_shuffle() {
    // 25 Generate a random permutation of the elements of a list.
    let data = vec![1, 2, 3, 4, 5];
    let res = lists::shuffle(data.clone());
    for i in res {
        assert_eq!(data.contains(&i), true);
    }
}

#[test]
fn test_combinations() {
    // 26 Generate the combinations of K distinct objects chosen from the N elements of a list
    let data = vec![1, 2, 3];
    let length = 2;
    let res = lists::combinations(length, data);
    let expected = vec![vec![1, 2], vec![1, 3], vec![2, 3]];
    assert_eq!(expected, res);

    let data = vec![1, 2, 3];
    let length = 3;
    let res = lists::combinations(length, data);
    let expected = vec![vec![1, 2, 3]];
    assert_eq!(expected, res);

    let data = vec![1, 2, 3, 4];
    let length = 3;
    let res = lists::combinations(length, data);
    let expected = vec![vec![1, 2, 3], vec![1, 2, 4], vec![1, 3, 4], vec![2, 3, 4]];
    assert_eq!(expected, res);
}

#[test]
fn test_combination() {
    // 27 Group the elements of a set into disjoint subsets.
//    let people = vec!["aldo", "beat", "carla", "david", "evi", "flip", "gary", "hugo", "ida"];
//    let res = lists::combination(vec![2, 3, 4], people);
//    assert_eq!(res[0],
//               vec![vec!["aldo", "beat"],
//                    vec!["carla", "david", "evi"],
//                    vec!["flip", "gary", "hugo", "ida"]]);
}

#[test]
fn test_lsort() {
    // 28 Sorting a list of lists according to length of sublists
    let data = vec![vec![1, 2, 3], vec![1, 2], vec![1, 2, 3, 4], vec![1]];
    let expected = vec![vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 2, 3, 4]];
    let res = lists::lsort(data);
    assert_eq!(res, expected);

    let data = vec![vec![1, 2, 3], vec![1, 2], vec![1, 2, 3], vec![1]];
    let expected = vec![vec![1], vec![1, 2], vec![1, 2, 3], vec![1, 2, 3]];
    let res = lists::lsort(data);
    assert_eq!(res, expected);
}