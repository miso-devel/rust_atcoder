use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32,
        x:i32
    }
    let mut ans = 0;
    for g in 0..=a {
        for h in 0..=b {
            for gg in 0..=c {
                if (g * 500 + h * 100 + gg * 50) == x {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans)
}
