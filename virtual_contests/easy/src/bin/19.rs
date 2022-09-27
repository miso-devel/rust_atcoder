use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:f32,
        b:f32
    }
    println!("{}", b / 100.0 * a)
}
