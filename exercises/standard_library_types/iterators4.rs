// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.


pub fn factorial(num: u64) -> u64 {
    //完成此功能以返回数字的阶乘
    // 不使用：
    // - 返回
    //尽量不要使用：
    //-命令风格循环（适用于，while）
    //  - 其他变量
    //为了额外的挑战，请不要使用：
    //  - 递归
    // Execute `rustlings hint iterators4` for hints.
    (1..=num).into_iter().product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
