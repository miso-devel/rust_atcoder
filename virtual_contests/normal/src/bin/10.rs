use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        w:usize,
        h:usize,
        n:usize,
        xy:[(usize,usize,usize);n]
    }
    let mut min_xy = (0, 0);
    let mut max_xy = (w, h);
    for (x, y, a) in xy {
        match a {
            1 => min_xy.0 = min_xy.0.max(x),
            2 => max_xy.0 = max_xy.0.min(x),
            3 => min_xy.1 = min_xy.1.max(y),
            4 => max_xy.1 = max_xy.1.min(y),
            _ => (),
        };
    }
    println!(
        "{}",
        max_xy.0.saturating_sub(min_xy.0) * max_xy.1.saturating_sub(min_xy.1)
    );
}
