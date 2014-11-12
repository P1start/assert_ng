#![feature(phase)]

#[phase(plugin)]
extern crate assert_ng;

#[test]
#[ignore]
fn assert_with_message() {
    assert_ng!(1i == 1, "foo bar");
}

#[test]
fn test_complex_expression() {
    assert_ng!(1i + 1i == 2i);
    assert_ng!(1i + 1i != 1i);
    assert_ng!(1i + 1i >  1i);
    assert_ng!(1i + 1i >= 2i);
    assert_ng!(1i + 1i <  3i);
    assert_ng!(1i + 1i <= 3i);
}
