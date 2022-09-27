use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {mut s:String}
    println!(
        "{}",
        if s.chars().nth(s.len() - 1).unwrap() == 's' {
            s + "es"
        } else {
            s + "s"
        }
    )
}
