use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        p:i32,
        a:[i32;n]
    }

    let mut ans = 0;
    for i in a {
        if i < p {
            ans += 1
        }
    }

    println!("{}", ans)
}
