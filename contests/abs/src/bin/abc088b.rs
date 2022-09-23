use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
      mut a:[u32],
    }
    let mut alice = 0;
    let mut bob = 0;
    a.sort();
    let sorted = a.iter().rev();
    for (i, x) in sorted.enumerate() {
        if i % 2 == 0 {
            alice += x
        } else {
            bob += x
        }
    }
    let sums = [alice, bob];
    println!(
        "{}",
        (sums.iter().max().unwrap()) - (sums.iter().min().unwrap())
    )
}
