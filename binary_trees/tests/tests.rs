#![feature(box_patterns)]

extern crate binary_trees;

use binary_trees::Tree;


#[test]
fn test_cbal_tree() {
    let c: char = 'x';

    let res: Tree<&char> = binary_trees::cbal_tree(3, &c);
    println!("\n\nTree: 3 -> {:?}\n\n", &res);

    let res: Tree<&char> = binary_trees::cbal_tree(4, &c);
    println!("\n\nTree: 4 -> {:?}\n\n", &res);

    let res: Tree<&char> = binary_trees::cbal_tree(5, &c);
    println!("\n\nTree: 5 -> {:?}\n\n", &res);
}
