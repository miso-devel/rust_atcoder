use proconio::{fastout, input};
// https://kenkoooo.com/atcoder/#/contest/show/90a6c4bb-0d6a-4e6b-85ea-852f6eaab746
// https://atcoder.jp/contests/abc182/editorial/289
#[fastout]
fn main() {
    input! {
        n:usize,
        a:[i32;n]
    }
    let mut max = -1;
    let mut ans = -1;
    for v in 2..=1000 {
        let mut sum = 0;

        for i in a.iter() {
            // 割り切れる場合にsumに１たす
            if i % v == 0 {
                sum += 1;
                println!(
                    "{} % {} = {} 現在のsum:{} 現在のmax:{} 現在のans:{}",
                    i,
                    v,
                    i % v,
                    sum,
                    max,
                    ans
                );
                // 割り切れた数がmaxよりも多い時sumに置き換える
                // そのときの値を最大のgcd値する
                if max <= sum {
                    max = sum;
                    ans = v;
                }
            }
        }
    }
    println!("{}", ans)
}
