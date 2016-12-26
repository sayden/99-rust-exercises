// pub enum Tree<T> {
//     Branch(T, Box<Tree<T>>, Box<Tree<T>>),
//     Empty(T),
// }


#[derive(Debug)]
pub enum Tree<T> {
    Branch(Box<Tree<T>>, Box<Tree<T>>),
    Empty(T),
}

pub fn cbal_tree<'a, T>(n: usize, data: &'a T) -> Vec<Tree<&'a T>> {
    if n == 0 {
        return vec![];
    }

    let r = (n - 1) % 2;
    let q = ((n - 1) / 2) as u32;

    let mut res = Vec::new();

    for i in q..(q + r as u32) {
        let left = cbal_tree(i as usize, data);
        for j in q..(q + r as u32) {
            let right = cbal_tree(n - i as usize - 1, data);
            res.push(Tree::Branch(Box::new(left), Box::new(right)));
        }
    }

    res
}
