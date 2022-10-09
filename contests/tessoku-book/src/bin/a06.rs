use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        q:i32,
        a:[i32;n],
        l:[(usize,usize);q]
    }
    // 累積和問題
    for (start, end) in l {
        // 配列の中にある特定の区間を切り出したものをslicedに格納
        let sliced = a.iter().as_slice().get((start - 1)..=(end - 1)).unwrap();
        // 合計を求める
        let sum: i32 = sliced.iter().sum();
        println!("{:?}", sum);
    }
}
