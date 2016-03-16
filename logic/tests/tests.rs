extern crate logic;

#[test]
fn test_table() {
    assert_eq!(logic::not(true), false);
    assert_eq!(logic::not(false), true);
    assert_eq!(logic::and(true, true), true);
    assert_eq!(logic::and(true, false), false);
    assert_eq!(logic::or(true, false), true);

    let res = logic::table(|x, y| logic::and(x, logic::or(x, y)));

    assert_eq!(res,
               "true true true\ntrue false true\nfalse true false\nfalse false false\n");
}
