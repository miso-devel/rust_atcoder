// 基本的な再帰関数
fn recursive(k: i32) -> i32 {
    if k == 0 {
        1
    } else {
        k * f(k - 1)
    }
}
