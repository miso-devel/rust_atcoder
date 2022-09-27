use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {s:i32,t:i32}
    let mut ans = 0;
    for j in 0..=s {
        for k in 0..=s {
            for l in 0..=s {
                if (j + k + l <= s) && (j * k * l <= t) {
                    ans += 1
                }
            }
        }
    }
    println!("{}", ans)
}
