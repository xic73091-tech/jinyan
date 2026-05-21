/// 记号定义 —— 诚言编程语言

use std::collections::HashMap as 哈希映射;

/// 源码位置
#[derive(Debug, Clone)]
pub struct 源位置 {
    pub 文件名: String,
    pub 行号: usize,
    pub 列号: usize,
}

impl 源位置 {
    pub fn 新建(文件名: &str, 行号: usize, 列号: usize) -> Self {
        源位置 { 文件名: 文件名.to_string(), 行号, 列号 }
    }
}

impl std::fmt::Display for 源位置 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.文件名, self.行号, self.列号)
    }
}

/// 记号种类
#[derive(Debug, Clone, PartialEq)]
pub enum 记号种类 {
    // 字面量
    整数字面量(i64),
    小数字面量(f64),
    文本字面量(String),
    标识符(String),

    // 关键字
    让, 定义, 恒定, 如果, 否则如果, 否则,
    当, 循环, 从, 到, 执行,
    对, 每项, 在, 中,
    返回, 尝试, 捕获, 抛出, 跳出, 继续,
    类, 继承, 构造, 方法, 类型, 结构, 导入,
    虚无, 整数, 小数, 文本, 布尔, 数组, 字典,
    本, 真, 假, 且, 或, 非,
    匹配, 分支, 新建, 公有, 私有,

    // 运算符
    加, 减, 乘, 除, 取余,
    等于, 不等于, 大于, 小于, 大于等于, 小于等于,
    赋值, 加赋值, 减赋值, 乘赋值, 除赋值,
    左圆括号, 右圆括号, 左花括号, 右花括号, 左方括号, 右方括号,
    点, 逗号, 冒号, 分号, 箭头, 管道,

    文件结束,
}

/// 记号
#[derive(Debug, Clone)]
pub struct 记号 {
    pub 种类: 记号种类,
    pub 位置: 源位置,
}

impl 记号 {
    pub fn 新建(种类: 记号种类, 位置: 源位置) -> Self {
        记号 { 种类, 位置 }
    }
}

/// 关键字表
pub fn 关键字表() -> 哈希映射<String, 记号种类> {
    let mut 表 = 哈希映射::new();
    表.insert("让".into(), 记号种类::让);
    表.insert("定义".into(), 记号种类::定义);
    表.insert("恒定".into(), 记号种类::恒定);
    表.insert("如果".into(), 记号种类::如果);
    表.insert("否则如果".into(), 记号种类::否则如果);
    表.insert("否则".into(), 记号种类::否则);
    表.insert("当".into(), 记号种类::当);
    表.insert("对".into(), 记号种类::对);
    表.insert("每项".into(), 记号种类::每项);
    表.insert("在".into(), 记号种类::在);
    表.insert("中".into(), 记号种类::中);
    表.insert("返回".into(), 记号种类::返回);
    表.insert("尝试".into(), 记号种类::尝试);
    表.insert("捕获".into(), 记号种类::捕获);
    表.insert("抛出".into(), 记号种类::抛出);
    表.insert("跳出".into(), 记号种类::跳出);
    表.insert("继续".into(), 记号种类::继续);
    表.insert("类".into(), 记号种类::类);
    表.insert("继承".into(), 记号种类::继承);
    表.insert("构造".into(), 记号种类::构造);
    表.insert("方法".into(), 记号种类::方法);
    表.insert("类型".into(), 记号种类::类型);
    表.insert("结构".into(), 记号种类::结构);
    表.insert("导入".into(), 记号种类::导入);
    // 类型名不作为关键字保留，允许作为变量名
    表.insert("本".into(), 记号种类::本);
    表.insert("真".into(), 记号种类::真);
    表.insert("假".into(), 记号种类::假);
    表.insert("且".into(), 记号种类::且);
    表.insert("或".into(), 记号种类::或);
    表.insert("非".into(), 记号种类::非);
    表.insert("匹配".into(), 记号种类::匹配);
    表.insert("分支".into(), 记号种类::分支);
    表.insert("新建".into(), 记号种类::新建);
    表.insert("公有".into(), 记号种类::公有);
    表.insert("私有".into(), 记号种类::私有);
    表
}

pub fn 是标识符首字符(c: char) -> bool {
    c.is_alphabetic() || c == '_' || 是中文字符(c)
}

pub fn 是标识符字符(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || 是中文字符(c)
}

pub fn 是中文字符(c: char) -> bool {
    matches!(c, '\u{4E00}'..='\u{9FFF}' | '\u{3400}'..='\u{4DBF}' | '\u{F900}'..='\u{FAFF}')
}
