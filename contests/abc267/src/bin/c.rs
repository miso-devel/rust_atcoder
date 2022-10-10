use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[isize;n]
    }
    // 下準備ーーーーーーーーーーーーーーーーーーー
    // 累積和配列の作成
    let mut sums = vec![0; n + 1];
    for i in 0..n {
        sums[i + 1] = sums[i] + a[i];
    }
    // 計算結果の初期値
    let mut sum = 0;
    for i in 0..m {
        sum += (i as isize + 1) * a[i];
    }
    // ーーーーーーーーーーーーーーーーーーー
    let mut ans = sum;
    for i in 0..(n - m) {
        // 今回追加する値のidx（+mして現在の値からどの値を追加するのかを判別）
        let r = i + m;
        let m = m as isize;
        println!(
            "sum:{} - ( sums:{} - sums:{} ) + ({}) === ans:{} --- result:{}",
            sum,
            sums[r],
            sums[i],
            m * a[r],
            ans,
            sum - (sums[r] - sums[i]) + m * a[r]
        );
        println!("{} & {}", a[i], a[r]);
        // 前回の計算結果 - 前回の総和（区間内） + (今回追加された値 * m)
        sum = sum - (sums[r] - sums[i]) + m * a[r];

        // ansの方が大きければ
        ans = ans.max(sum);
    }
    println!("{:?}", ans)
}
