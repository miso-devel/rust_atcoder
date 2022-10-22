use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:i32,
        w:usize,
        c:[String;h]
    }
    let mut res: Vec<i32> = vec![0; w];
    for i in 0..h {
        for t in 0..w {
            let str = c[i as usize].chars().nth(t).unwrap();
            if str.to_string() == "#" {
                res[t] += 1;
            }
        }
    }
    for a in 0..res.iter().len() {
        if a == 0 {
            print!("{}", res[a as usize])
        } else {
            print!(" {}", res[a as usize])
        }
    }
}
