use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut n:usize,
        a:[usize;n]
    }
    let mut ans = [0; n];
    let mut cloned = a.clone();
    cloned.sort();
    cloned.dedup();
    println!("a:{:?}", a);
    println!("cloned:{:?}", cloned);
    let mut count = Ok(10);
    let mut t_count: Vec<usize> = vec![];
    for i in a {
        count = cloned.binary_search(&i);
        if count.is_ok() {
            t_count.push(cloned.len() - 1 - count.unwrap())
        } else {
            t_count.push(count.unwrap_err())
        }
        println!("i:{}  count:{:?}", i, t_count);
    }

    for t in t_count {}
}
