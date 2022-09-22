use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s:String
    }
    let vec: Vec<char> = s.chars().collect();
    let mut result: i64 = 0;
    let match_pattern: char = '1';
    for v in vec {
        if v == match_pattern {
            result += 1;
        };
    }
    println!("{}", result)
}
