use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {mut list:[u32]}
    let mut ans = 0;
    while list.iter().all(|&l| l % 2 == 0) {
        list.iter_mut().for_each(|l: &mut u32| *l /= 2);
        ans += 1;
    }
    println!("{}", ans)
}
