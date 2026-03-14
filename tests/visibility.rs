use piperize::piperize;

#[piperize]
pub fn pub_double(a: i32) -> i32 {
    a * 2
}

#[piperize]
pub(crate) fn crate_double(a: i32) -> i32 {
    a * 2
}

#[piperize]
fn private_double(a: i32) -> i32 {
    a * 2
}

#[test]
fn public_visibility_works() {
    assert_eq!(6, 3.pub_double());
}

#[test]
fn crate_visibility_works() {
    assert_eq!(8, 4.crate_double());
}

#[test]
fn private_visibility_inside_module() {
    assert_eq!(10, 5.private_double());
}

#[test]
fn visibility_compile_tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/visibility/pass_public.rs");
    t.compile_fail("tests/visibility/fail_private.rs");
}
