use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        c:char
    }
    println!("{}", (c as u8 + 1) as char);
}
