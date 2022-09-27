use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
    s1:String,
    s2:String,
    s3:String,
    t:String
    }
    let strs = [s1, s2, s3];
    let mut ans = "".to_string();
    for v in 0..t.len() {
        let idx = t.chars().nth(v).unwrap() as usize;
        let s = strs[idx - 49].as_str();
        ans.push_str(s);
    }
    println!("{}", ans)
}
