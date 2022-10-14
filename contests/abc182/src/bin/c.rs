use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:String
    }
    // let mut ans: usize = n.len();
    let mut is_ans = 0;
    let mut ans: usize = 10000000000000000000;

    for i in 0..1 << n.len() {
        for j in 0..(n.len()) {
            if (1 << j) & i == 0 {
                // println!("{}", n.chars().nth(j).unwrap());
                let num: isize = n.chars().nth(j).unwrap().to_string().parse().unwrap();
                // println!("{}", num);
                if (num / 3) == 0 {
                    is_ans += 1;
                    ans = ans.min(n.chars().nth(j).unwrap().to_string().len() as usize);
                    // println!("{}", ans)
                };
            }
        }
    }
    let num: isize = n.parse().unwrap();
    if (num / 3) == 0 && is_ans == 0 {
        println!("-1");
    } else if (num / 3) == 0 {
        println!("0")
    } else {
        println!("{}", ans);
    }
}
