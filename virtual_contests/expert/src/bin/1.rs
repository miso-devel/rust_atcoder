use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        t:i128,
        ts:[i128;n]
    }
    let mut ans = t;
    for p in 0..n - 1 {
        // tより小さい場合は次の値との差分を＋する。
        // t秒たった後に来たならtを足す。
        // 例）4秒経つ前、3秒地点でボタンを押すと3秒地点からお湯が4秒で始める
        // ただ4秒地点は前押したボタンによって出ているお湯がまだ出ているので5~7秒の間に3秒地点で押したボタンが出ることとなる。

        // println!(
        //     "現在 : {}  ts: {} 結果: {} 現在のans : {}",
        //     p,
        //     ts[p as usize],
        //     (ts[(p + 1) as usize] - ts[p as usize]).min(t),
        //     ans
        // );
        ans += (ts[(p + 1) as usize] - ts[p as usize]).min(t);
    }
    println!("{}", ans)
}
