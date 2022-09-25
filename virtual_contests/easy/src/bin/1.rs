use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32
    }
    println!("{}", if n == 0 { 1 } else { 0 })
}
