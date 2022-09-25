use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x:i32
    }
    if x < 1200 {
        println!("ABC")
    } else {
        println!("ARC")
    }
}
