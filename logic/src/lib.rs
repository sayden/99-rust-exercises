// 46. and 47 Define predicates and/2, or/2, nand/2, nor/2, xor/2, impl/2 and equ/2
// (for logical equivalence) which succeed or fail according to the result of their
// respective operations; e.g. and(A,B) will succeed, if and only if both A and B succeed.
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


// 49 Gray codes.
pub fn gray(n: u32) -> Vec<String> {
    match n {
        0 => vec![],
        1 => vec!["0".to_string(), "1".to_string()],
        n => {
            let mut res: Vec<String> = Vec::new();
            for i in gray(n - 1) {
                let mut a = "0".to_string();
                a.push_str(i.as_str());
                res.push(a);
                let mut b = "1".to_string();
                b.push_str(i.as_str());
                res.push(b);
            }
            res.sort();
            return res;
        }
    }
}
