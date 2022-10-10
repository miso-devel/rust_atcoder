use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        q:i32,
        a:[i32;n],
        l:[(usize,usize);q]
    }
    // 累積和の前計算
    let mut custom_array: Vec<i32> = vec![0];
    // custom_arrayに総和をpushしていく
    for i in 1..=n {
        custom_array.push(custom_array[(i - 1) as usize] + a[(i - 1) as usize])
    }
    // q回で計算終わり
    // ..endの値から..startの値を引けば総和が求められる
    for (start, end) in l {
        println!("{}", custom_array[end] - custom_array[start - 1])
    }
}
