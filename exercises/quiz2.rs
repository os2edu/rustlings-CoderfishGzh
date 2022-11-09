// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

//让我们以功能形式构建一台小机器。
//作为输入，我们将提供字符串和命令列表。这些命令
//确定将采用哪种操作应用于字符串。可以是：
//-大写字符串
//-修剪字符串
//-将“ bar”附加到字符串上指定的次数
//确切的形式将是：
//-输入将是2个长度元组的向量，
//第一个元素是字符串，第二个元素是命令。
//-输出元素将是字符串的向量。
// No hints this time!



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            let s = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(size) => {
                    let mut s = string.clone();
                    for _ in 0..*size as i32 {
                        s.push_str("bar");
                    }
                    s.to_string()
                }
            };

            output.push(s);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we have to import to have `transformer` in scope?
    use my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
