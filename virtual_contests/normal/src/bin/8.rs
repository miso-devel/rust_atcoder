use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h:usize,
        w:usize,
        a:[String;h]
    }
    for i in 0..h {
        if h == 1 {
            println!("{}", "#".repeat(w + 2));
            println!("#{}#", a[i]);
            println!("{}", "#".repeat(w + 2));
        } else {
            if i == 0 {
                println!("{}", "#".repeat(w + 2));
                println!("#{}#", a[i]);
            } else if i == h - 1 {
                println!("#{}#", a[i]);
                println!("{}", "#".repeat(w + 2))
            } else {
                println!("#{}#", a[i])
            }
        }
    }
}
