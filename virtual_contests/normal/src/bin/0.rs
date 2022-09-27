use proconio::{fastout, input};
// https://kenkoooo.com/atcoder/#/contest/show/be8c9ad1-b128-4f16-9f41-e7a1f9309e29?activeTab=Problems
#[fastout]
fn main() {
    input! {a:i32,b:i32}

    println!(
        "{}",
        [
            (a % 10 + (a % 100) / 10 + (a % 1000) / 100),
            (b % 10 + (b % 100) / 10 + (b % 1000) / 100)
        ]
        .iter()
        .max()
        .unwrap()
    );
}
