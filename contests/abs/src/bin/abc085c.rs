use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i128,
        y:i128
    }
    let mut ans = [-1, -1, -1];
    for a in 0..=n {
        for b in 0..=n - a {
            let c = n - a - b;
            let sum = a * 10000 + b * 5000 + c * 1000;
            if sum == y {
                ans = [a, b, c]
            }
        }
    }

    println!("{} {} {}", ans[0], ans[1], ans[2])
}
