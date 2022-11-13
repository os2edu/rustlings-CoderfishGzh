// from_str.rs
//这类似于from_into.rs，但是这次我们将实现``fromstr''
//并返回错误，而不是落回默认值。
//此外，在实现从Str的实施后，您可以使用“解析方法”
//在字符串上生成实施者类型的对象。
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a hint.

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}


// 脚步：
// 1.如果提供的字符串的长度为0，则应返回错误
// 2.将给定的字符串分开在其中存在的逗号上
// 3.只能从分折中返回2个元素，否则返回错误
// 4.从拆分操作中提取第一个元素并将其用作名称
// 5.从拆分操作中提取其他元素，并将其分析为“ usize”
//带有``4“” .parse :: <usize>（）``''
// 6.如果提取名称和年龄出现问题时，应返回错误
//如果一切顺利，请返回一个人对象的结果
//
//作为旁边的：`box <dyn error>`从<＆'_ str>`实现`。这意味着，如果您想返回
//字符串错误消息，您可以通过使用返回`err（“我的错误消息” .into（））````''

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(ParsePersonError::Empty);
        }

        let tmp: Vec<&str> = s.split(',').collect();

        if tmp.len() != 2 {
            return Err(ParsePersonError::BadLen);
        }

        if tmp[0] == "" {
            return Err(ParsePersonError::NoName);
        }

        let name = tmp[0];
        let age = match tmp[1].parse::<usize>() {
            Ok(a) => a,
            Err(e) => return Err(ParsePersonError::ParseInt(e)),
        };

        Ok(Person {
            name: name.to_string(),
            age,
        })


    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(ParsePersonError::Empty));
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    fn missing_age() {
        assert!(matches!(
            "John,".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!(
            "John,twenty".parse::<Person>(),
            Err(ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(ParsePersonError::NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(
            ",".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(ParsePersonError::NoName | ParsePersonError::ParseInt(_))
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(ParsePersonError::BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!(
            "John,32,man".parse::<Person>(),
            Err(ParsePersonError::BadLen)
        );
    }
}
