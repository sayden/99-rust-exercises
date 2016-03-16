extern crate logic;

use logic::{not, and, or, table, gray};

#[test]
fn test_table() {
    // 46. and 47 Define predicates and/2, or/2, nand/2, nor/2, xor/2, impl/2 and equ/2
    // (for logical equivalence) which succeed or fail according to the result of their
    // respective operations; e.g. and(A,B) will succeed, if and only if both A and B succeed.
    assert_eq!(not(true), false);
    assert_eq!(not(false), true);
    assert_eq!(and(true, true), true);
    assert_eq!(and(true, false), false);
    assert_eq!(or(true, false), true);

    let res = table(|x, y| and(x, or(x, y)));

    assert_eq!(res,
               "true true true\ntrue false true\nfalse true false\nfalse false false\n");
}


#[test]
fn test_gray_codes() {
    // 49 Gray codes.
    assert_eq!(gray(2), vec!["00", "01", "10", "11"]);
    assert_eq!(gray(3),
               vec!["000", "001", "010", "011", "100", "101", "110", "111"]);
    assert_eq!(gray(4),
               vec!["0000", "0001", "0010", "0011", "0100", "0101", "0110", "0111", "1000",
                    "1001", "1010", "1011", "1100", "1101", "1110", "1111"]);
}
