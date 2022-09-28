use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:String,
        b:String
    }
    let num: f64 = format!("{}{}", a, b).parse().unwrap();
    println!(
        "{}",
        if num.sqrt() == (num.sqrt() as i32 as f64) {
            "Yes"
        } else {
            "No"
        }
    )
}
