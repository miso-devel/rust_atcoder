use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l:usize,
        r:usize
    }
    println!("{}", &"atcoder"[l - 1..r])
}
