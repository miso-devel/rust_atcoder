use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut mountain: [(String, i32); n],
    }
    mountain.sort_by(|a, b| (-b.1).partial_cmp(&(-a.1)).unwrap());
    println!("{}", mountain[n - 2].0)
}
