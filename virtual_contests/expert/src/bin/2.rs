use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      a:isize,
      b:isize,
      x:isize
    }
    // println!("a%x : {}    b/x : {}    a/x : {}", a % x, b / x, a / x);
    let ans = if a % x == 0 { 1 } else { 0 } + b / x - a / x;
    println!("{}", ans);
}
