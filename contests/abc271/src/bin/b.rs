use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        q:i32,
        l:[[i32];n],
        s:[(usize,usize);q]
    }

    for v in s {
        let idx = v.0 - 1;
        let idx2 = v.1 - 1;
        println!("{}", l[idx][idx2])
    }
}
