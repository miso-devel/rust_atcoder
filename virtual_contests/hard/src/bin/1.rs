use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        t:[i32;n],
        m:usize,
        p:[(usize,i32);m]
    }
    for i in p {
        let mut nums = t.clone();
        let idx = i.0 - 1;
        nums[idx] = i.1;
        let ans: i32 = nums.iter().sum();
        println!("{}", ans)
    }
}
