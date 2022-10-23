use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:String
    }
    let ary: Vec<_> = n.split("").filter(|&x| x != "").collect();
    let mut ans: isize = n.len() as isize;
    let mut cnt = 0;
    for bit in 1..(1 << n.len()) {
        let sub_list: Vec<_> = (0..n.len())
            .filter(|x| (bit & (1 << x)) != 0)
            .map(|x| ary[x])
            .collect();
        let str = sub_list
            .iter()
            .map(|s| s.trim())
            .collect::<Vec<_>>()
            .join("");
        let num: isize = str.parse().unwrap();
        let num_length: isize = str.len() as isize;
        if num % 3 == 0 {
            ans = ans.min((n.len() as isize) - num_length);
            cnt += 1;
        }
    }
    if cnt == 0 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
