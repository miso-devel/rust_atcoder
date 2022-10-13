use proconio::{fastout, input};
// https://kenkoooo.com/atcoder/#/contest/show/99925854-1bd4-4254-abee-cc12386afd91?activeTab=Standings
#[fastout]
fn main() {
    input! {
        s:[String;3],
        t:[String;3]
    }
    let ary1: Vec<Vec<String>> = vec![
        vec!["R".to_string(), "G".to_string(), "B".to_string()],
        vec!["B".to_string(), "R".to_string(), "G".to_string()],
        vec!["G".to_string(), "B".to_string(), "R".to_string()],
    ];

    let case1 = ary1.iter().any(|ary| ary == &s);
    let case2 = ary1.iter().any(|ary| ary == &t);
    if case1 || case2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
