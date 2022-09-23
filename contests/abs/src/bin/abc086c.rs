use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        q: [(isize, isize, isize); n]
    }
    println!("0番目 : {} {} {}", q[0].0, q[0].1, q[0].2);
    for v in q {
        println!("{} {} {}", v.0, v.1, v.2);
    }
}
