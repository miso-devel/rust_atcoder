use proconio::{fastout, input};

#[fastout]
fn f(k: i32) -> i32 {
    if k == 0 {
        1
    } else {
        k * f(k - 1)
    }
}
fn main() {
    input! {
        k:i32
    }
    println!("{}", f(k));
}
