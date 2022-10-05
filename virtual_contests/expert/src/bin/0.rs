use proconio::{fastout, input};
// https://kenkoooo.com/atcoder/#/contest/show/87da4c2e-504e-4a92-9211-7dbb54c394c2?activeTab=Problems

#[fastout]
fn main() {
    input! {
        n:i32,
        a:[i32;n]
    }
    let mut ans = 1000000;
    for v in -100..=100 {
        let mut res = 0;
        for i in a.iter() {
            res += (i - v).pow(2);
            // println!("i:{} - v:{} .pow:{}", i, v, res);
        }
        // println!("{}", res);
        if res < ans {
            ans = res;
        }
    }
    println!("{}", ans)
}
