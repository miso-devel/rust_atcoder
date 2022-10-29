use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        h:[i32;n]
    }
    let mut ans: i32 = 0;
    let mut res: i32 = 0;
    for t in 0..n {
        if res <= h[t as usize] {
            res = h[t as usize];
            ans = t;
        };
    }
    println!("{}", ans + 1);
}
