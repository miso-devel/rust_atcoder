use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:i32,
        _w:i32,
        c:[String;h]
    }
    for v in c {
        println!("{}", v);
        println!("{}", v)
    }
}
