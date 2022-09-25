use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x:i32
    }
    if 30 <= x {
        println!("Yes")
    } else {
        println!("No")
    }
}
