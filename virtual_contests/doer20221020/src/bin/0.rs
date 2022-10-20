use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut v:i32,
        a:i32,
        b:i32,
        c:i32
    }
    loop {
        if v - a < 0 {
            println!("F");
            break;
        }
        v -= a;
        if (v - b) < 0 {
            println!("M");
            break;
        }
        v -= b;

        if (v - c) < 0 {
            println!("T");
            break;
        }
        v -= c;
    }
}
