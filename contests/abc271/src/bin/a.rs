use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:i32
    }
    if n < 16 {
        println!("0{}", format!("{:X}", n));
    } else {
        println!("{}", format!("{:X}", n));
    }
}
