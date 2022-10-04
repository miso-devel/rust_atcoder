use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[String;n]
    }
    let mut ans = "correct";
    for j in 0..n {
        for i in 0..n {
            let res = (a[j].chars().nth(i).unwrap(), a[i].chars().nth(j).unwrap());
            match res {
                ('W', 'L') => (),
                ('L', 'W') => (),
                ('-', '-') => (),
                ('D', 'D') => (),
                _ => ans = "incorrect",
            }
        }
    }
    println!("{}", ans)
}
