use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t:i32,
        case:[(i32,i32,String);t]
    }
    // まず条件にマッチするパターンが作れる文字列なのか
    // ここの条件分岐
    // その後、そのパターンにマッチした文字列が1回だけなのか、それ以上ならNo
    for (n, k, s) in case {
        println!("n:{} k:{} s:{}", n, k, s)
    }
}
