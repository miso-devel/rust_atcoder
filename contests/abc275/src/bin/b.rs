use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:isize,
        b:isize,
        c:isize,
        d:isize,
        e:isize,
        f:isize,
    }
    let tmp: Option<isize> = ((a * b * c) - (d * e * f)).checked_add(0);
    let dd: isize = 998244353;
    if tmp != None {
        println!("{}", tmp.unwrap() % dd);
    } else {
        println!("0");
    }
}
