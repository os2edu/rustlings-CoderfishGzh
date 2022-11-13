// try_from_into.rs
// Tryfrom是一种简单且安全的类型转换，在某些情况下可能会以受控的方式失败。
//基本上，这与来自。主要区别在于，这应该返回结果类型
//而不是目标类型本身。
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}



//您的任务是完成此实施
//并返回内部类型颜色的确定结果。
//您需要为三个整数的元组创建实现，
//一个三个整数和一个整数。
//
//请注意，元组和数组的实现将在编译时检查，
//但是切片实现需要检查切片长度！
//还要注意，正确的RGB颜色值必须在0 .. = 255范围内是整数。

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let red = tuple
            .0
            .try_into()
            .map_err(|_| IntoColorError::IntConversion)?;
        
        let green = tuple
            .1
            .try_into()
            .map_err(|_| IntoColorError::IntConversion)?;

        let blue = tuple
            .2
            .try_into()
            .map_err(|_| IntoColorError::IntConversion)?;


        if vaild(red) && vaild(green) && vaild(blue) {
            return Ok(Color {
                red,
                green,
                blue,
            })
        }

        Err(IntoColorError::IntConversion)
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        Color::try_from((arr[0], arr[1], arr[2]))
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(IntoColorError::BadLen);
        }

        Color::try_from((slice[0], slice[1], slice[2]))
    }
}

fn vaild(rbg: u8) -> bool {
    if rbg < 0 || rbg > 255 {
        return false;
    }
    true
}

fn main() {
    // Use the `try_from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since TryFrom is implemented for Color, we should be able to use TryInto
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `try_from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use TryInto
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
}
