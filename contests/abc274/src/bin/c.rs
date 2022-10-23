use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        a:[isize;n]
    }
    println!("{:?}", a);
    let res = vec![0; (2 * n + 1) as usize];
}
