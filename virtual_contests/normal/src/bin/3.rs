use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[i32;n]
    }

    let mut ans = a
        .iter()
        .enumerate()
        .map(|(idx, value)| (value, idx + 1))
        .collect::<Vec<_>>();

    ans.sort();
    println!("{}", ans[a.len() - 2].1);
}
