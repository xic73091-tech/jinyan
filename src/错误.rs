/// 错误类型 —— 诚言编程语言

use std::fmt;

/// 位置类型别名，统一使用记号模块中的源位置
pub type 位置 = crate::记号::源位置;

/// 词法错误
#[derive(Debug)]
pub enum 词法错误 {
    非法字符(char, 位置),
    未闭合字符串(位置),
    非法数字(位置),
}

impl fmt::Display for 词法错误 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            词法错误::非法字符(c, p) => write!(f, "{}: 非法字符 '{}'", p, c),
            词法错误::未闭合字符串(p) => write!(f, "{}: 未闭合的字符串", p),
            词法错误::非法数字(p) => write!(f, "{}: 数字格式不合法", p),
        }
    }
}

/// 语法错误
#[derive(Debug)]
pub enum 语法错误 {
    意外记号 { 期望: String, 实际: String, 位置: 位置 },
    缺少记号 { 期望: String, 位置: 位置 },
    无效表达式(位置),
}

impl fmt::Display for 语法错误 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            语法错误::意外记号 { 期望, 实际, 位置 } =>
                write!(f, "{}: 期望 '{}'，实际 '{}'", 位置, 期望, 实际),
            语法错误::缺少记号 { 期望, 位置 } =>
                write!(f, "{}: 缺少 '{}'", 位置, 期望),
            语法错误::无效表达式(p) => write!(f, "{}: 无效的表达式", p),
        }
    }
}

/// 编译错误
#[derive(Debug)]
pub enum 编译错误 {
    未定义变量(String, 位置),
}

impl fmt::Display for 编译错误 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            编译错误::未定义变量(n, p) => write!(f, "{}: 未定义的变量 '{}'", p, n),
        }
    }
}

/// 运行时错误
#[derive(Debug)]
pub enum 运行时错误 {
    类型错误(String),
    除以零,
    索引越界(usize, usize),
    空引用,
    栈溢出,
}

impl fmt::Display for 运行时错误 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            运行时错误::类型错误(m) => write!(f, "类型错误: {}", m),
            运行时错误::除以零 => write!(f, "运行时错误: 除以零"),
            运行时错误::索引越界(i, l) => write!(f, "索引越界: 索引 {} 超出范围 [0, {})", i, l),
            运行时错误::空引用 => write!(f, "运行时错误: 空引用"),
            运行时错误::栈溢出 => write!(f, "运行时错误: 调用栈溢出"),
        }
    }
}

/// 从String转换为运行时错误（值类型的运算方法返回Result<值, String>）
impl From<String> for 运行时错误 {
    fn from(消息: String) -> Self {
        运行时错误::类型错误(消息)
    }
}
