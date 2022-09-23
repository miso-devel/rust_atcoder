use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        a:i32,
        b:i32
    }
    let mut ans = 0;
    for i in 0..=n {
        let condition =
            i % 10 + (i / 10) % 10 + (i / 100) % 10 + (i / 1000) % 10 + (i / 10000) % 10;
        if a <= condition && b >= condition {
            ans += i
        }
    }

    println!("{}", ans)
}
