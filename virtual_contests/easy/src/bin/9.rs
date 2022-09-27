use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n:[i32;3]
    }
    n.sort();
    if n[2] - n[1] == n[1] - n[0] {
        println!("Yes");
    } else {
        println!("No");
    }
}
