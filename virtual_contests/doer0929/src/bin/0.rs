use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[usize;w];h]
    }
    for v in 0..w {
        for i in 0..h {
            if i == h - 1 {
                print!("{}", a[i][v])
            } else {
                print!("{} ", a[i][v])
            }
        }
        print!("\n");
    }
}
