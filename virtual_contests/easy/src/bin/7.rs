use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s:String
    }
    let mut ans = 0;
    for n in s.chars() {
        if n == '+' {
            ans += 1
        } else {
            ans -= 1
        }
    }
    println!("{}", ans)
}
