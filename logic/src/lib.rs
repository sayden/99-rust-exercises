pub fn not(i: bool) -> bool {
    !i
}

pub fn and(x: bool, y: bool) -> bool {
    x && y
}

pub fn or(x: bool, y: bool) -> bool {
    x || y
}

pub fn nor(x: bool, y: bool) -> bool {
    !or(x, y)
}

pub fn nand(x: bool, y: bool) -> bool {
    !and(x, y)
}

pub fn xor(x: bool, y: bool) -> bool {
    !equ(x, y)
}

pub fn equ(x: bool, y: bool) -> bool {
    x == y
}

pub fn table<F>(f: F) -> String
    where F: Fn(bool, bool) -> bool
{
    print_binary(f, vec![true, false])
}

fn print_binary<A: std::fmt::Display + Copy, B: std::fmt::Display, F>(f: F, vec: Vec<A>) -> String
    where F: Fn(A, A) -> B
{

    let mut res = Vec::new();

    for i in &vec {
        for j in &vec {
            res.push(format!("{} {} {}\n", i, j, f(*i, *j)));
        }
    }

    res.into_iter().fold("".to_string(), |acc, b| acc + b.as_str())
}
