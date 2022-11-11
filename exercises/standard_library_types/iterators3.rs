// iterators3.rs
//这比其他大多数都更大！你能行的！
//这是您的使命，如果您选择接受：
// 1.完成分隔函数以获得前四个测试。
// 2.通过完成result_with_list和
// list_of_results函数。
// Execute `rustlings hint iterators3` or use the `hint` watch subcommand for a hint

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

//计算`a“ a”除以`a“ a”的均匀分开。
//否则，返回合适的错误。
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {

    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }

    let ret = a / b;
    if ret * b != a {
        return Err(DivisionError::NotDivisible(NotDivisibleError{
            dividend: a,
            divisor: b,
        }));
    }

    Ok(a / b)
}

// 完成功能并返回正确类型的值，以使测试通过。
// Desired output: Ok([1, 11, 1426, 3])
fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results = numbers.into_iter().map(|n| divide(n, 27)).collect();
    division_results
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    let division_results: Vec<Result<i32, DivisionError>> = numbers.into_iter().map(|n| divide(n, 27)).collect();
    division_results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible(NotDivisibleError {
                dividend: 81,
                divisor: 6
            }))
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        assert_eq!(format!("{:?}", result_with_list()), "Ok([1, 11, 1426, 3])");
    }

    #[test]
    fn test_list_of_results() {
        assert_eq!(
            format!("{:?}", list_of_results()),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}
