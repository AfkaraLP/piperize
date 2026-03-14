use piperize::piperize;

#[piperize]
fn double(a: i32) -> i32 {
    2 * a
}

#[piperize]
fn many_args(a: i32, b: String, c: u64) -> &'static str {
    println!("{}{b}", a as u64 + c);
    "thing workes"
}

#[piperize]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[piperize]
fn to_stringified(a: i32) -> String {
    format!("{a}")
}

#[test]
fn basic_test() {
    assert_eq!(4, 2.double());
}

#[test]
fn zero_case() {
    assert_eq!(0, 0.double());
}

#[test]
fn negative_numbers() {
    assert_eq!(-6, (-3).double());
}

#[test]
fn chaining_calls() {
    assert_eq!(8, 2.double().double());
}

#[test]
fn add_pipe() {
    assert_eq!(7, 3.add(4));
}

#[test]
fn add_then_double() {
    assert_eq!(14, 3.add(4).double());
}

#[test]
fn string_return() {
    assert_eq!("42", 42.to_stringified());
}

#[test]
fn many_args_call() {
    let res = 5.many_args("abc".to_string(), 7);
    assert_eq!("thing workes", res);
}

#[test]
fn many_args_different_values() {
    let res = 10.many_args("zzz".to_string(), 100);
    assert_eq!("thing workes", res);
}

#[test]
fn chaining_with_conversion() {
    let s = 21.double().to_stringified();
    assert_eq!("42", s);
}
