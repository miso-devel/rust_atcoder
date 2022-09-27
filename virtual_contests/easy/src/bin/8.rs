use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n:[i32;3]
    }
    n.sort();

    println!("{}", n[1] + n[2]);
}
