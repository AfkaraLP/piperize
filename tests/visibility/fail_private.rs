use piperize::piperize;

mod inner {
    use super::*;

    #[piperize]
    fn double(a: i32) -> i32 {
        a * 2
    }
}

fn main() {
    let _ = 3.double();
}
