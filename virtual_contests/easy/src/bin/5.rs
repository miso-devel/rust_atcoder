use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x:[u32;5]
    }
    let i = x.iter().position(|&x| x == 0).unwrap();
    println!("{}", i + 1);
}
