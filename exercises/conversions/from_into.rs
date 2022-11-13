//从特征用于价值转换。
//如果从某种类型中正确实现，则应相反地工作。
//您可以在https://doc.rust-lang.org/std/convert/trait.from.html上阅读更多有关它的信息。
//执行```Rustlings提示''_into`或使用``hint'''Watch subcmand command进行提示。

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// We implement the Default trait to use it as a fallback
// when the provided string is not convertible into a Person object
impl Default for Person {
    fn default() -> Person {
        Person {
            name: String::from("John"),
            age: 30,
        }
    }
}

//您的任务是完成此实施
//为了使行`让p = person :: from（“ mark，20”）`汇编
//请注意，您需要将年龄组件分解为“ usize”
//带有``4“” .parse :: <usize>（）`。这种结果需要
//适当处理。
//
// 脚步：
// 1.如果提供的字符串的长度为0，则返回人的默认值
// 2.将给定的字符串分开在其中存在的逗号上
// 3.从拆分操作中提取第一个元素并将其用作名称
// 4.如果名称为空，请返回人的默认值
// 5.从拆分操作中提取其他元素，并将其分析为“ usize”
//如果在解析年龄时会出现问题，然后退还人的默认
//否则，然后返回带有结果的实例化对象


impl From<&str> for Person {
    fn from(s: &str) -> Person {

        if s.is_empty() {
            return Default::default();
        }

        let tmp: Vec<&str> = s.split(',').collect();


        if tmp.len() != 2 {
            return Default::default();
        }

        if tmp[0] == "" {
            return Default::default();
        }

        let name = tmp[0];
        let age = match tmp[1].parse::<usize>() {
            Ok(a) => a,
            Err(_) => return Default::default(),
        };

        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    // Use the `from` function
    let p1 = Person::from("Mark,20");
    // Since From is implemented for Person, we should be able to use Into
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default() {
        // Test that the default person is 30 year old John
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }
    #[test]
    fn test_bad_convert() {
        // Test that John is returned when bad string is provided
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
    #[test]
    fn test_good_convert() {
        // Test that "Mark,20" works
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }
    #[test]
    fn test_bad_age() {
        // Test that "Mark,twenty" will return the default person due to an error in parsing age
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,man");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
