use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        p:[u8;26]
    }
    for i in 0..26 {
        print!("{}", (p[i] + b'a' - 1) as char);
    }
    println!();
}
