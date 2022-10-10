use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d:usize,
        n:i32,
        l:[(i32,i32);n]
    }
    let mut custom_array = vec![0; d - 1];
    println!("{:?}", custom_array);
    // 前処理
    for (start, end) in l {
        for t in start..end {
            custom_array[t as usize] += 1;
        }
    }

    for c in custom_array {
        println!("{}", c)
    }
}
