use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l1:i32,
        r1:i32,
        l2:i32,
        r2:i32,
    }
    let mut ans = 0;
    for r in l1..r1 {
        for b in l2..r2 {
            if r == b {
                ans += 1
            }
        }
    }
    println!("{}", ans)
}
