// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 { // 看起来直接return也是可以的，但是不推荐
    // 默认返回最后一个表达式，需要类型和函数签名中相同即可
    num * num
}
