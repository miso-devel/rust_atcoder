use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:i32,
        b:i32,
        c:i32
    }
    let mut ans = -1;
    for i in a..=b {
        if i % c == 0 {
            ans = i;
            break;
        }
    }
    println!("{}", ans)
}
