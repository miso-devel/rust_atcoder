use num::integer;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:i32,
        x:[i32;n]
    }

    for v in x {
        for i in 0..=50 {
            println!("{}", integer::gcd(v, i))
        }
    }
}
