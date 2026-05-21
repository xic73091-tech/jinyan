/// 运行时值类型 —— 诚言编程语言

use std::collections::HashMap as 哈希映射;
use std::fmt;

/// 值枚举
#[derive(Debug, Clone)]
pub enum 值 {
    整数(i64),
    小数(f64),
    文本(String),
    布尔(bool),
    空值,
    数组(Vec<值>),
    映射(哈希映射<String, 值>),
    函数(函数对象),
    原生函数(原生函数对象),
    类(类定义),
    实例(实例对象),
}

#[derive(Debug, Clone)]
pub struct 函数对象 {
    pub 名称: String,
    pub 参数数量: u8,
    pub 局部变量数量: u8,
    pub 起始地址: usize,
}

#[derive(Clone)]
pub struct 原生函数对象 {
    pub 名称: String,
    pub 函数: fn(&[值]) -> 值,
}

impl fmt::Debug for 原生函数对象 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "原生函数({})", self.名称)
    }
}

#[derive(Debug, Clone)]
pub struct 类定义 {
    pub 名称: String,
    pub 方法: 哈希映射<String, 函数对象>,
    pub 父类: Option<String>,
}

#[derive(Debug, Clone)]
pub struct 实例对象 {
    pub 类名: String,
    pub 属性: 哈希映射<String, 值>,
}

impl 值 {
    pub fn 类型名(&self) -> &str {
        match self {
            值::整数(_) => "整数",
            值::小数(_) => "小数",
            值::文本(_) => "文本",
            值::布尔(_) => "布尔",
            值::空值 => "空",
            值::数组(_) => "数组",
            值::映射(_) => "映射",
            值::函数(_) => "函数",
            值::原生函数(_) => "原生函数",
            值::类(_) => "类",
            值::实例(_) => "实例",
        }
    }

    pub fn 转布尔(&self) -> bool {
        match self {
            值::整数(0) => false,
            值::整数(_) => true,
            值::小数(0.0) => false,
            值::小数(_) => true,
            值::文本(s) => !s.is_empty(),
            值::布尔(b) => *b,
            值::空值 => false,
            值::数组(a) => !a.is_empty(),
            值::映射(m) => !m.is_empty(),
            _ => true,
        }
    }

    pub fn 转文本(&self) -> String {
        match self {
            值::整数(n) => n.to_string(),
            值::小数(n) => n.to_string(),
            值::文本(s) => s.clone(),
            值::布尔(true) => "真".into(),
            值::布尔(false) => "假".into(),
            值::空值 => "空值".into(),
            值::数组(a) => {
                let 内容: Vec<String> = a.iter().map(|v| v.转文本()).collect();
                格式化!("[{}]", 内容.join(", "))
            }
            值::映射(m) => {
                let 内容: Vec<String> = m.iter().map(|(k, v)| 格式化!("{}: {}", k, v.转文本())).collect();
                格式化!("{{{}}}", 内容.join(", "))
            }
            值::函数(f) => 格式化!("函数<{}>", f.名称),
            值::原生函数(f) => 格式化!("原生函数<{}>", f.名称),
            值::类(c) => 格式化!("类<{}>", c.名称),
            值::实例(i) => 格式化!("{}实例", i.类名),
        }
    }

    pub fn 加(&self, 右: &值) -> Result<值, String> {
        match (self, 右) {
            (值::整数(a), 值::整数(b)) => a.checked_add(*b).map(值::整数).ok_or_else(|| "整数加法溢出".into()),
            (值::小数(a), 值::小数(b)) => Ok(值::小数(a + b)),
            (值::整数(a), 值::小数(b)) => Ok(值::小数(*a as f64 + b)),
            (值::小数(a), 值::整数(b)) => Ok(值::小数(a + *b as f64)),
            (值::文本(a), 值::文本(b)) => Ok(值::文本(format!("{}{}", a, b))),
            (值::文本(a), b) => Ok(值::文本(format!("{}{}", a, b.转文本()))),
            (a, 值::文本(b)) => Ok(值::文本(format!("{}{}", a.转文本(), b))),
            _ => Err(格式化!("不能对 {} 和 {} 执行加法", self.类型名(), 右.类型名())),
        }
    }

    pub fn 减(&self, 右: &值) -> Result<值, String> {
        match (self, 右) {
            (值::整数(a), 值::整数(b)) => a.checked_sub(*b).map(值::整数).ok_or_else(|| "整数减法溢出".into()),
            (值::小数(a), 值::小数(b)) => Ok(值::小数(a - b)),
            (值::整数(a), 值::小数(b)) => Ok(值::小数(*a as f64 - b)),
            (值::小数(a), 值::整数(b)) => Ok(值::小数(a - *b as f64)),
            _ => Err(格式化!("不能对 {} 和 {} 执行减法", self.类型名(), 右.类型名())),
        }
    }

    pub fn 乘(&self, 右: &值) -> Result<值, String> {
        match (self, 右) {
            (值::整数(a), 值::整数(b)) => a.checked_mul(*b).map(值::整数).ok_or_else(|| "整数乘法溢出".into()),
            (值::小数(a), 值::小数(b)) => Ok(值::小数(a * b)),
            (值::整数(a), 值::小数(b)) => Ok(值::小数(*a as f64 * b)),
            (值::小数(a), 值::整数(b)) => Ok(值::小数(a * *b as f64)),
            _ => Err(格式化!("不能对 {} 和 {} 执行乘法", self.类型名(), 右.类型名())),
        }
    }

    pub fn 除(&self, 右: &值) -> Result<值, String> {
        match (self, 右) {
            (_, 值::整数(0)) | (_, 值::小数(0.0)) => Err("除以零".into()),
            (值::整数(a), 值::整数(b)) => a.checked_div(*b).map(值::整数).ok_or_else(|| "整数除法溢出".into()),
            (值::小数(a), 值::小数(b)) => Ok(值::小数(a / b)),
            (值::整数(a), 值::小数(b)) => Ok(值::小数(*a as f64 / b)),
            (值::小数(a), 值::整数(b)) => Ok(值::小数(a / *b as f64)),
            _ => Err(格式化!("不能对 {} 和 {} 执行除法", self.类型名(), 右.类型名())),
        }
    }

    pub fn 取余(&self, 右: &值) -> Result<值, String> {
        match (self, 右) {
            (_, 值::整数(0)) => Err("取余除以零".into()),
            (值::整数(a), 值::整数(b)) => a.checked_rem(*b).map(值::整数).ok_or_else(|| "整数取余溢出".into()),
            _ => Err(格式化!("不能对 {} 和 {} 执行取余", self.类型名(), 右.类型名())),
        }
    }

    pub fn 取反(&self) -> Result<值, String> {
        match self {
            值::整数(n) => n.checked_neg().map(值::整数).ok_or_else(|| "整数取反溢出".into()),
            值::小数(n) => Ok(值::小数(-n)),
            _ => Err(格式化!("不能对 {} 取反", self.类型名())),
        }
    }

    pub fn 相等(&self, 右: &值) -> bool {
        match (self, 右) {
            (值::整数(a), 值::整数(b)) => a == b,
            (值::小数(a), 值::小数(b)) => a == b,
            (值::整数(a), 值::小数(b)) => (*a as f64) == *b,
            (值::小数(a), 值::整数(b)) => *a == (*b as f64),
            (值::文本(a), 值::文本(b)) => a == b,
            (值::布尔(a), 值::布尔(b)) => a == b,
            (值::空值, 值::空值) => true,
            _ => false,
        }
    }

    pub fn 小于(&self, 右: &值) -> Result<bool, String> {
        match (self, 右) {
            (值::整数(a), 值::整数(b)) => Ok(a < b),
            (值::小数(a), 值::小数(b)) => Ok(a < b),
            (值::整数(a), 值::小数(b)) => Ok((*a as f64) < *b),
            (值::小数(a), 值::整数(b)) => Ok(*a < (*b as f64)),
            (值::文本(a), 值::文本(b)) => Ok(a < b),
            _ => Err(格式化!("不能比较 {} 和 {}", self.类型名(), 右.类型名())),
        }
    }
}

impl fmt::Display for 值 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.转文本())
    }
}
