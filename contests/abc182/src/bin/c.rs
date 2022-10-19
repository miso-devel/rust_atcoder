use proconio::{fastout, input};
#[fastout]

fn to_digit(num: usize) -> String {
    format!("{:b}", num)
}
fn main() {
    input! {
        n:String
    }
    let mut to_array: Vec<_> = n.split("").collect();
    to_array.pop();
    println!("{:?}", to_array);
    let sam = [3, 6, 9, 5];
    println!("{}", 1 << 7);
    println!("{}", 1 & (1 << 2));
    // bit全探索
    // filterでは条件に合うものしか返さない
    for bit in 0..(1 << sam.len()) {
        let sample: Vec<_> = (0..sam.len()).filter(|x| (bit & (1 << x)) != 0).collect();
        let sub_list = (0..sam.len())
            .filter(|x| (bit & (1 << x)) != 0)
            .map(|x| sam[x]);

        println!("filter結果: {:?}", sample);
        println!("今回の組み合わせ   bit: {} の時", bit);

        sub_list.for_each(|x| {
            println!("x: {:?}", x);
        });
        println!("\n")
    }
}
