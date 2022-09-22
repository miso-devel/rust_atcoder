use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
    }
    if n == 1 || n >= 5 {
        println!("Yes")
    } else {
        println!("No");
    }
}
