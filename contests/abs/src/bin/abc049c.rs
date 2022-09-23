use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut s:String,
    }
    let option = ["eraser", "erase", "dreamer", "dream"];
    for str in option.iter() {
        s = s.replace(str, "").to_string();
    }

    if s == "" {
        println!("YES");
    } else {
        println!("NO");
    }
}
