use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:i32,
        m:i32,
        mut t:isize,
        mut a:[isize;n-1],
        xy:[(usize,isize);m]
    }
    let mut ans = "Yes";
    for (x, y) in xy {
        a[x - 1] -= y;
    }

    for v in a {
        t -= v;
        if t <= 0 {
            ans = "No";
            break;
        }
    }
    println!("{}", ans)
}
