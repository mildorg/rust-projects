use practice::testing::doc_testing;
mod common;

#[test]
fn test_add() {
    common::step();
    assert_eq!(doc_testing::add(3, 2), 5);
}
