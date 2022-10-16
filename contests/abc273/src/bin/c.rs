use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n:usize,
        a:[usize;n]
    }
    let mut cloned = a.clone();
    cloned.sort();
    cloned.dedup();
    let mut count = Ok(10);
    let mut t_count: Vec<usize> = vec![0; n];
    for i in a {
        count = cloned.binary_search(&i);
        if count.is_ok() {
            let ok = cloned.len() - 1 - count.unwrap();
            t_count[ok] += 1;
        } else {
            let err = count.unwrap_err();
            t_count[err] += 0;
        }
    }

    for t in t_count {
        println!("{}", t)
    }
}
