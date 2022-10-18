use num_traits::Pow;
use proconio::{fastout, input};
fn selected_num(num: f64, position: u8) -> f64 {
    // 割る数
    let division: f64 = 10.0.pow(position);
    let ans = num / division;
    ans
}

#[fastout]
fn main() {
    input! {
        mut x:f64,
        k:isize
    }
    println!("{}", selected_num(2345.0, 2));
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
