// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.



// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn byte_counter<T : AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}
/*
arg.as_ref()：需要返回能调用 as_bytes() 的类型。
as_bytes()：&str 的方法，返回 &[u8]。
len()：返回字节数。
T 必须能转换为 &str。

T: AsRef<str>：T 可以是 &str、String 等，能转换为 &str。
arg.as_ref()：返回 &str。
as_bytes()：返回字节切片。
len()：字节数。
*/

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.
fn char_counter<T : AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}
/*
arg.as_ref()：需要返回能调用 chars() 的类型。
chars()：&str 的方法，返回字符迭代器。
count()：计算字符数。
T 必须能转换为 &str。

同 byte_counter，AsRef<str> 确保 T 可转为 &str。
chars()：处理 Unicode 字符（如 é 算 1 个字符）。
count()：返回总数。
*/

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T : AsMut<u32>>(arg: &mut T) {
    let num = arg.as_mut();
    *num = *num * *num;
}
/*
arg: &mut T：可变引用，需修改值。
测试用例：Box<u32> 从 3 变为 9。
需要 as_mut() 转换为可变数字类型（如 &mut u32）。
平方操作：*arg = *arg * *arg。
T 必须能转换为可变数字类型，且支持乘法。

T: AsMut<u32>：T 可以是 Box<u32>、直接 u32 等，能转为 &mut u32。
arg.as_mut()：返回 &mut u32。
*num * *num：平方。
*num = ...：更新值。
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
