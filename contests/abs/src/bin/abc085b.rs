use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        mut list:[u32]
    }
    list.sort();
    list.dedup();
    println!("{}", list.len());
}
