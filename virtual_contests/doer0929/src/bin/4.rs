use num::integer;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:i32,
        x:[i32;n]
    }
    // bit全探索
    // https://www.try-it.jp/chapters-4962/sections-4963/lessons-5016/
    // https://atcoder.jp/contests/arc114/tasks/arc114_a
    for v in x {
        for i in 0..=50 {
            println!("{}", integer::gcd(v, i))
        }
    }
}
