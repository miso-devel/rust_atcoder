use itertools::Itertools;
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:usize,
        mut p:[i32;n],
        mut q:[i32;n],
    }
    // for文でするとN!ループ回すことになる
    // 順列全探索
    let min = p.iter().min().unwrap();
    let max = p.iter().max().unwrap();
    let mut a = 0;
    let mut b = 0;
    for perm in (*min..=*max).permutations(n) {
        a += 1;
        if p == perm {
            break;
        }
    }

    for perm in (*min..=*max).permutations(n) {
        b += 1;
        if q == perm {
            break;
        }
    }
    let ans = a - b;
    println!("{}", num_traits::abs(ans));
}
