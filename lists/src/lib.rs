#![feature(slice_patterns)]
#![feature(default_type_parameter_fallback)]

extern crate rand;

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
pub fn pack<T: Copy + PartialEq>(vec: Vec<T>) -> Vec<Vec<T>> {
    let mut res: Vec<Vec<T>> = vec![];

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
            res.push(first_list);

            // First group, now we must group second part
            let second = xs.iter()
                           .skip_while(|y| **y == x)
                           .collect::<Vec<&T>>();
            let mut second_list: Vec<T> = Vec::new();
            for i in second {
                second_list.push(*i);
            }

            let mut temp = pack(second_list);
            res.append(&mut temp);

        }
        _ => return res,
    }

    res
}

// 10 Run-length encoding of a list.
pub fn encode<T: Copy + PartialEq>(vec: Vec<T>) -> Vec<(usize, T)> {
    let packed = pack(vec);

    packed.iter()
          .map(|group| (group.len(), group[0]))
          .collect::<Vec<(usize, T)>>()
}

// 11 Modified run-length encoding.
#[derive(Debug, PartialEq, Clone)]
pub enum EncodeType<T: Copy + PartialEq> {
    Single(T),
    Multiple(usize, T),
}

pub fn encode_modified<T: Copy + PartialEq>(vec: Vec<T>) -> Vec<EncodeType<T>> {
    let packed: Vec<Vec<T>> = pack(vec);

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

// 12 Decode a run-length encoded list.
pub fn decode_modified<T: Copy + PartialEq>(vec: Vec<EncodeType<T>>) -> Vec<T> {
    let mut res: Vec<T> = vec![];
    for i in vec {
        match i {
            EncodeType::Single(e) => res.push(e),
            EncodeType::Multiple(n, e) => {
                for _ in 0..n {
                    res.push(e);
                }
            }
        }
    }
    res
}

// 13 Run-length encoding of a list (direct solution).
pub fn encode_direct<T: Copy + PartialEq>(vec: Vec<T>) -> Vec<(usize, T)> {
    let mut res: Vec<(usize, T)> = vec![];

    match vec.as_slice() {
        [x] => res.push((1usize, x)),
        [x, xs..] => {
            let first = xs.iter()
                          .take_while(|y| **y == x)
                          .collect::<Vec<&T>>();

            res.push((first.len() + 1, x));

            // First group, now we must group second part
            let second = xs.into_iter()
                           .skip_while(|y| **y == x)
                           .collect::<Vec<&T>>();
            let mut second_list: Vec<T> = Vec::new();
            for i in second {
                second_list.push(*i);
            }
            let mut temp = encode_direct(second_list);
            res.append(&mut temp);
        }
        _ => return res,
    }

    res
}

// 14 Duplicate the elements of a list.
pub fn dupli<T: Copy>(vec: Vec<T>) -> Vec<T> {
    vec.into_iter().flat_map(|x| vec![x, x]).collect()
}


// 15 Replicate the elements of a list a given number of times.
pub fn repli<T: Copy>(vec: Vec<T>, n: usize) -> Vec<T> {
    vec.into_iter()
       .flat_map(|x| {
           vec![x]
               .into_iter()
               .cycle()
               .take(n)
       })
       .collect()
}

// 16 Drop every N'th element from a list.
pub fn drop<T: Copy>(vec: Vec<T>, n: usize) -> Vec<T> {
    let mut res = vec![];
    if n == 1 || n == 0 || vec.len() == 0 {
        return res;
    }

    let length = vec.len();

    for i in 1..length + 1 {
        if (i % n) != 0 {
            res.push(vec[i - 1]);
        }
    }

    res
}

// 17 Split a list into two parts; the length of the first part is given.
pub fn split<T: Copy>(vec: Vec<T>, index: usize) -> Vec<Vec<T>> {
    vec![vec.clone().into_iter().take(index).collect(), vec.into_iter().skip(index).collect()]
}

// 18 Extract a slice from a list.
pub fn slice<T: Copy>(vec: Vec<T>, start: usize, end: usize) -> Vec<T> {
    vec.into_iter().skip(start - 1).take(end - start + 1).collect()
}

// 19 Rotate a list N places to the left.
pub fn rotate<T: Copy>(vec: Vec<T>, places: usize) -> Vec<T> {
    vec.clone().into_iter().skip(places).chain(vec.into_iter().take(places)).collect()
}

// 20 Remove the K'th element from a list.
pub fn remove<T: Copy>(vec: Vec<T>, n: usize) -> Vec<T> {
    vec.clone().into_iter().take(n - 1).chain(vec.into_iter().skip(n)).collect()
}

// 21 Insert an element at a given position into a list.
pub fn insert_at<T: Copy>(pos: usize, vec: Vec<T>, elem: T) -> Vec<T> {
    let mut res: Vec<T> = vec![];

    for (i, v) in vec.iter().enumerate() {
        if i == pos - 1 {
            res.push(elem);
        }

        res.push(*v);
    }

    res
}

// 22 Create a list containing all integers within a given range.
pub fn range(start: usize, end: usize) -> Vec<u32> {
    (start..end + 1).into_iter().map(|x| x as u32).collect()
}

// 23 Extract a given number of randomly selected elements from a list.
use rand::{thread_rng, Rng};

pub fn rnd_select<T: Copy>(vec: Vec<T>, amount: usize) -> Vec<T> {
    let mut res: Vec<T> = Vec::new();

    let mut count = 0;
    while count < amount {
        let rnd = thread_rng().gen_range::<usize>(0, amount);
        res.push(vec[rnd]);
        count += 1
    }
    res
}
