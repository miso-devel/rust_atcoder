use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32
    }
    let mut s = 0;
    for i in n.to_string().chars() {
        s += i as i32 - 48
    }
    println!("{}", if (n % s) == 0 { "Yes" } else { "No" })
}
