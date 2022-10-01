use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x:i32,
        mut y:i32,
        mut z:i32
    }
    // 障害物地点が０以下にあるなら全てに−１をかけて正の方向に持っていく
    // bit orわからないとできない問題
    // https://atcoder.jp/contests/abc270/editorial/4877
    if y < 0 {
        x *= -1;
        y *= -1;
        z *= -1
    }
    // xゴール地点、y障害物地点、zハンマー地点
    // スタート地点からゴール地点に障害物がある場合
    if (0..x).contains(&y) {
        // スタート地点からハンマー地点に障害物がある場合
        if (0..z).contains(&y) {
            // ハンマー地点までにyがある場合はゴールまで行けないので−１
            println!("-1")
        } else {
            // それ以外はzまでの距離＋zからx地点までいった距離
            println!("{}", (z.abs() + (z - x).abs()))
        }
    } else {
        println!("{}", x.abs())
    }
}
