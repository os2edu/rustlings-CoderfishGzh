// quiz3.rs
// This quiz tests:
// - Generics
// - Traits
//一个想象中的魔法学校有一个新的成绩单生成系统，用Rust编写！
//当前系统仅支持创建学生成绩的成绩单
//以数值表示（例如1.0-> 5.5）。
//但是，学校还发布字母等级（a+  - > f-）和需求
//能够打印两种类型的成绩单！

//在结构ReportCard和Impl Block中进行必要的代码更改
//支持字母表报告卡。将第二个测试中的等级更改为“ A+”
//表明您的更改允许字母顺序排列。

// Execute `rustlings hint quiz3` or use the `hint` watch subcommand for a hint.



pub struct ReportCard<T: std::fmt::Display> {
    pub grade: T,
    pub student_name: String,
    pub student_age: u8,
}

impl<T: std::fmt::Display> ReportCard<T> {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, &self.grade)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: "A+",
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
