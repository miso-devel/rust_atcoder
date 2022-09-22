use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        p: i32,
        q: i32,
        r: i32,
    }
    let array: [i32; 3];
    array = [p + q, p + r, q + r];
    print!("{}", array[0]);
    println!("{}", array.iter().min().unwrap())
}
