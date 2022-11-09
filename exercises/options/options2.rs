// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        };
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: 让它一会儿说明 - 请记住，vector.pop还添加了另一层选项<t>
        //您可以将``option <t>'s''放在一起，如果让
        while let Some(integer) = optional_integers.pop() {
            assert_eq!(integer, Some(range));
            range -= 1;
        }
    }
}
