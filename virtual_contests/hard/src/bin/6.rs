use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(usize,usize);m]
    }
    let mut ans = 0;
    // 頂点と頂点が繋がっているかをまとめる二次元配列
    let mut relatives = vec![vec![false; n]; n];
    // 頂点uと繋がっているvの間に辺があるとする（trueとする）
    for (u, v) in uv {
        // println!("頂点：{}   繋がっている頂点：{}", u, v);
        relatives[u - 1][v - 1] = true;
    }
    // println!("{:?}", relatives);
    // 無向グラフ問題
    // i,f,tが仮に1,3,5だとするとその三つの頂点が繋がっていることを決め打ちした上で全探索
    for i in 0..n {
        for f in 0..n {
            for t in 0..n {
                if relatives[i][f] && relatives[f][t] && relatives[i][t] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans)
}
