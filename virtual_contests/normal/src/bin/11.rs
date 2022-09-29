use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        s:[String;n]
    }

    if s.iter().any(|v| v == "Y") {
        println!("{}", "Four")
    } else {
        println!("{}", "Three")
    }
}
