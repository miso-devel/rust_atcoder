use proconio::{fastout, input};
#[fastout]

fn to_digit(num: usize) -> String {
    format!("{:b}", num)
}
fn main() {
    input! {
        n:String
    }
    let length = n.len();
    let mut digit_ary: Vec<Vec<usize>> = vec![];
    for i in 0..(1 << length) {
        let mut digits: Vec<usize> = vec![];
        for j in 0..length {
            if (1 << j) & i == 0 {
                digits.push(j as usize)
            }
        }
        digit_ary.push(digits);
    }
    digit_ary.pop();
    println!("{:?}", digit_ary);
}
