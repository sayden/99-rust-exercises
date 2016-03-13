#![feature(slice_patterns)]
#![feature(default_type_parameter_fallback)]

// 1 Find the last element of a list.
pub fn my_last<T: Clone>(vec: Vec<T>) -> T {
    let mut reversed = vec;
    reversed.reverse();
    reversed[0].clone()
}

// 2 Find the last but one element of a list.
pub fn my_but_last<T: Clone>(vec: Vec<T>) -> T {
    let mut reversed = vec;
    reversed.reverse();
    reversed[1].clone()
}

// 3 Find the K'th element of a list. The first element in the list is number 1
pub fn element_at<T: Clone>(vec: Vec<T>, pos: usize) -> T {
    vec[pos - 1].clone()
}

// 4 Find the number of elements of a list.
pub fn my_length<T>(vec: Vec<T>) -> usize {
    let mut size = 0;
    for _ in &vec {
        size += 1;
    }

    size
}

// 5 Reverse a list
pub fn my_reverse<T: Clone>(vec: Vec<T>) -> Vec<T> {
    let mut r = Vec::with_capacity(vec.len());
    for e in &vec {
        r.insert(0, e.clone())
    }

    r
}

// 6 Find out whether a list is a palindrome. A palindrome can be read forward or backward; e.g. (x a m a x).
pub fn is_palindrome<T: PartialEq>(vec: Vec<T>) -> bool {
    let middle;
    if vec.len() % 2 == 0 {
        middle = (vec.len() / 2) - 1;

    } else {
        middle = (vec.len() - 1 / 2) - 1;
    }

    let mut tail = vec;
    let head = tail.split_off(middle);

    let mut i = 0;
    for e in head {
        if e != tail[i] {
            return false;
        }
        i += 1;
    }

    return true;
}

// 7 Flatten a nested list structure.
pub enum NestedList<T: PartialEq> {
    Elem(T),
    List(Vec<NestedList<T>>),
}

pub fn my_flatten<T: PartialEq>(b: Vec<NestedList<T>>, acc: &mut Vec<T>) {
    for i in b.into_iter() {
        match i {
            NestedList::Elem(e) => acc.push(e),
            NestedList::List(nl) => my_flatten(nl, acc),
        }
    }
}

// 8 Eliminate consecutive duplicates of list elements. dedup()
pub fn compress<T: PartialEq + Copy>(vec: Vec<T>, acc: &mut Vec<T>) {
    match vec.as_slice() {
        [x] => acc.push(x),

        [x, xs..] => {
            acc.push(x);
            let rest = xs.iter()
                         .skip_while(|y| x == **y)
                         .collect::<Vec<&T>>();
            let mut new_list: Vec<T> = vec![];
            for i in rest {
                new_list.push(*i);
            }

            compress(new_list, acc);
        }

        _ => return,
    }
}

// 9 Pack consecutive duplicates of list elements into sublists
pub fn pack<T: Copy + PartialEq>(vec: Vec<T>, acc: &mut Vec<Vec<T>>) {
    match vec.as_slice() {
        [x, xs..] => {
            let first = xs.iter()
                          .take_while(|y| **y == x)
                          .collect::<Vec<&T>>();

            let mut first_list: Vec<T> = Vec::new();
            first_list.push(x);
            for i in first {
                first_list.push(*i);
            }
            acc.push(first_list);

            // First group, now we must group second part
            let second = xs.iter()
                           .skip_while(|y| **y == x)
                           .collect::<Vec<&T>>();
            let mut second_list: Vec<T> = Vec::new();
            for i in second {
                second_list.push(*i);
            }

            pack(second_list, acc);
        }
        _ => return,
    }
}

// 10 Run-length encoding of a list.
pub fn encode<T: Copy + PartialEq>(vec: Vec<T>) -> Vec<(usize, T)> {
    let mut packed: Vec<Vec<T>> = Vec::new();
    pack(vec, &mut packed);

    packed.iter()
          .map(|group| (group.len(), group[0]))
          .collect::<Vec<(usize, T)>>()
}

// 11 Modified run-length encoding.
#[derive(Debug, PartialEq, Clone)]
pub enum EncodeType<T> {
    Single(T),
    Multiple(usize, T),
}

pub fn encode_modified<T: Copy + PartialEq>(vec: Vec<T>) -> Vec<EncodeType<T>> {
    let mut packed: Vec<Vec<T>> = Vec::new();
    pack(vec, &mut packed);

    packed.iter()
          .map(|group| (group.len(), group[0]))
          .map(|t| {
              if t.0 == 1 {
                  EncodeType::Single(t.1)
              } else {
                  EncodeType::Multiple(t.0, t.1)
              }
          })
          .collect::<Vec<EncodeType<T>>>()
}
