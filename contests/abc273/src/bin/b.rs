use num_traits::Pow;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut x:f64,
        k:isize
    }
    // println!("{}", x);
    for i in 0..=k {
        // 割る数
        let g = 10.0.pow(i as u8);
        // println!("割る数：{}", g);
        let num: f64 = (x / g) as f64;
        // println!("割られた数：{}", num);
        x = num.round() * g;
        // println!("元に戻した数：{}", x)
    }
    println!("{}", x);
}
