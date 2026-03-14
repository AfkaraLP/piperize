use std::time::Duration;

use piperize::piperize;
use tokio::time::sleep;

#[piperize]
async fn double_async(a: i32) -> i32 {
    2 * a
}

#[piperize]
async fn many_args_async(a: i32, b: String, c: u64) -> &'static str {
    println!("{}{b}", a as u64 + c);
    "thing workes"
}

#[piperize]
async fn add_async(a: i32, b: i32) -> i32 {
    a + b
}

#[piperize]
async fn to_stringified_async(a: i32) -> String {
    format!("{a}")
}

#[piperize]
async fn many_awaits(a: i32) -> i32 {
    sleep(Duration::from_millis(1)).await;
    let x = a * 2;

    sleep(Duration::from_millis(1)).await;
    let y = x + 3;

    sleep(Duration::from_millis(1)).await;
    y * 2
}
#[tokio::test]
async fn basic_async_test() {
    assert_eq!(4, 2.double_async().await);
}

#[tokio::test]
async fn zero_case_async() {
    assert_eq!(0, 0.double_async().await);
}

#[tokio::test]
async fn negative_numbers_async() {
    assert_eq!(-6, (-3).double_async().await);
}

#[tokio::test]
async fn chaining_calls_async() {
    assert_eq!(8, 2.double_async().await.double_async().await);
}

#[tokio::test]
async fn add_pipe_async() {
    assert_eq!(7, 3.add_async(4).await);
}

#[tokio::test]
async fn add_then_double_async() {
    let res = 3.add_async(4).await.double_async().await;
    assert_eq!(14, res);
}

#[tokio::test]
async fn string_return_async() {
    assert_eq!("42", 42.to_stringified_async().await);
}

#[tokio::test]
async fn many_args_call_async() {
    let res = 5.many_args_async("abc".to_string(), 7).await;
    assert_eq!("thing workes", res);
}

#[tokio::test]
async fn many_args_different_values_async() {
    let res = 10.many_args_async("zzz".to_string(), 100).await;
    assert_eq!("thing workes", res);
}

#[tokio::test]
async fn chaining_with_conversion_async() {
    let s = 21.double_async().await.to_stringified_async().await;
    assert_eq!("42", s);
}

#[tokio::test]
async fn many_awaits_test() {
    assert_eq!(14, 2.many_awaits().await);
}
