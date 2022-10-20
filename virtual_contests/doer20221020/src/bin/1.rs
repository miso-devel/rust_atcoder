use std::string;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:i32,
        mut s:Chars
    }
    let mut count = 0;
    println!("{:?}", s);
    s.dedup();
    println!("{:?}", s);

    for i in s {
        println!("{}", i);
    }
}
