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

#[derive(Clone)]
pub struct HNode {
    val: usize,
    data: HTree,
}

#[derive(Clone)]
pub enum HTree {
    Leaf(char),
    Branch(Branch),
}

#[derive(Clone)]
pub struct Branch {
    left: Box<HNode>,
    right: Box<HNode>,
}

impl HNode {
    pub fn new_leaf(val: usize, c: char) -> Self {
        HNode {
            val: val,
            data: HTree::Leaf(c),
        }
    }

    pub fn new_branch(a: HNode, b: HNode) -> Self {
        HNode {
            val: a.val + b.val,
            data: HTree::Branch(Branch {
                left: Box::new(a),
                right: Box::new(b),
            }),
        }
    }

    fn get_tree(vec: Vec<HNode>) -> Self {
        let debug = false;

        let mut sorted: Vec<HNode> = vec.clone();

        sorted.sort_by(|a, b| a.val.cmp(&b.val));

        if debug {
            println!("Init list is:");

            for i in &sorted {
                println!("{},{}",
                         i.val,
                         match i.data {
                             HTree::Leaf(c) => c,
                             _ => '*',
                         });
            }

            println!("");
        }

        // Ready
        let mut done = false;
        while !done {

            let candidates = sorted.clone().into_iter().take(2).collect::<Vec<HNode>>();
            sorted = sorted.clone().into_iter().skip(2).collect::<Vec<HNode>>();

            // Create new, insert and sort
            let new = HNode::new_branch(candidates[0].clone(), candidates[1].clone());
            sorted.push(new);
            sorted.sort_by(|a, b| a.val.cmp(&b.val));


            if debug {
                println!("");

                for i in &sorted {
                    println!("{},{}",
                             i.val,
                             match i.data {
                                 HTree::Leaf(c) => c,
                                 _ => '*',
                             });
                }
            }

            if !(sorted.len() > 1) {
                done = true;
            }
        }

        if debug {
            println!("Finished!");
        }

        return sorted[0].clone();
    }

    pub fn find_char(&self, c: char, side: String) -> String {
        match &self.data {
            &HTree::Leaf(ref _c) => {
                if c == *_c {
                    return side;
                }

                return "".to_string();
            }
            &HTree::Branch(ref b) => {
                let mut _side = side.clone();
                _side.push('0');
                let left = b.left.find_char(c, _side);
                if left.len() > 0 {
                    return left;
                }

                let mut _side = side.clone();
                _side.push('1');
                let right = b.right.find_char(c, _side);
                if right.len() > 0 {
                    return right;
                }

                return "".to_string();
            }
        }
    }
}


pub fn huffman(vec: Vec<(char, usize)>) -> Vec<(char, String)> {
    let mut nodes: Vec<HNode> = Vec::new();

    for i in vec.clone() {
        nodes.push(HNode::new_leaf(i.1, i.0));
    }

    let tree = HNode::get_tree(nodes);

    let mut res = Vec::new();
    for i in vec {
        let s: String = tree.find_char(i.0, "".to_string());
        res.push((i.0, s));
    }

    res
}
