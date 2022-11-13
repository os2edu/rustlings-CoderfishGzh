// clippy1.rs
// Clippy工具是一系列棉花来分析您的代码
//因此，您可以捕获常见的错误并改善生锈代码。
//
//对于这些练习
//检查输出的Clippy建议以解决练习。
//执行`rustlings提示clippy1`或使用``提示''Watch subcmand conf to提示。



use std::f32;

fn main() {
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
