use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
    mut p:[usize;n]
    }
    // 1 問目の配点は A 点以下、2 問目の配点は A+1 点以上 B 点以下、3 問目の配点は B+1 点以上である

    let mut tmp1 = 0;
    let mut tmp2 = 0;
    let mut tmp3 = 0;
    for v in p {
        if v <= a {
            tmp1 += 1
        }
        if v >= a + 1 && v <= b {
            tmp2 += 1
        }
        if v >= b + 1 {
            tmp3 += 1
        }
    }

    println!("{}", [tmp1, tmp2, tmp3].iter().min().unwrap())
}
