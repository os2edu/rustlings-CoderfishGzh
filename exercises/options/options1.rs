// options1.rs
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a hint.



// 此功能返回冰箱中剩下多少冰淇淋。
//如果是晚上10点之前，还有5件。晚上10点，有人吃了
//全部，所以不再有:(
// TODO: Return an Option!
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
   //我们在这里使用24小时系统，因此10pm的值为22和12am是0
    //选项输出应优雅处理time_of_day> 23的情况。
    if time_of_day >= 22 && time_of_day <= 24{
        return Some(0);
    } 
    if time_of_day > 0 && time_of_day < 22 {
        return Some(5);
    } 

    return None;
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams, Some(5));
    }
}
