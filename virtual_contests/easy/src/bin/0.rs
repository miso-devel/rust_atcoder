use proconio::{fastout, input};
// https://kenkoooo.com/atcoder/#/contest/show/6d4ca2a5-4d9d-4265-afd9-c6588ae2566b?activeTab=Problems
#[fastout]
fn main() {
    input! {
        n:i32,
        a:i32,
        b:i32,
    }
    println!("{}", n - a + b)
}
