// box1.rs
//
//在编译时，Rust需要知道一种类型占用多少空间。这变得有问题
//对于递归类型，该值可以作为本身的另一个值的一部分。
//为了解决这个问题，我们可以使用“盒”  - 用于将数据存储在堆上的智能指针，
//这也使我们可以包装递归类型。
//
//我们在本练习中实施的递归类型是“ cons列表”  - 数据结构
//经常在功能编程语言中找到。弊端列表中的每个项目都包含两个
//元素：当前项目和下一个项目的值。最后一个项目是称为“ nil”的值。
//
//步骤1：在枚举定义中使用`box'制作代码编译
//步骤2：通过替换`todo！
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.


#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {

    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(0, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
