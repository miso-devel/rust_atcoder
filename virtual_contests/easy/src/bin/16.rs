use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        a:[i32;n]
    }
    let mut ans = 0;
    for i in a {
        if i > 10 {
            ans += i - 10
        }
    }
    println!("{}", ans)
}
