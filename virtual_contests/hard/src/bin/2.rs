use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        s:String
    }
    let mut ans = 0;
    for i in 0..n {
        let abcs = b'a'..=b'z';
        let (f, e) = s.split_at(i);
        let chars_f: Vec<char> = f.chars().collect();
        let chars_e: Vec<char> = e.chars().collect();
        let mut res = 0;
        for v in abcs {
            let vv = v as char;
            if chars_e.contains(&vv) && chars_f.contains(&vv) {
                res += 1;
            }
        }
        if ans < res {
            ans = res
        }
    }
    println!("{}", ans)
}
