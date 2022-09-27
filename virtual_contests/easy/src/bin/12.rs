use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        s:String
    }
    let weather = s.chars().nth(n - 1).unwrap() == 'o';
    if weather {
        println!("Yes")
    } else {
        println!("No")
    }
}
