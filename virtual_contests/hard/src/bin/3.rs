use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:i32,
        a:[i32;n]
    }
    let max = a.iter().max().unwrap();
    let min = a.iter().min().unwrap();
    let ans = max - min;
    println!("{}", ans.abs())
}
