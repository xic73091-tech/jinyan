/// 词法分析器 —— 诚言编程语言

use crate::记号::*;
use crate::错误::词法错误;

pub struct 词法分析器 {
    源码: Vec<char>,
    当前位置: usize,
    行号: usize,
    列号: usize,
    文件名: String,
    关键字表: std::collections::HashMap<String, 记号种类>,
}

impl 词法分析器 {
    pub fn 新建(源码: &str, 文件名: &str) -> Self {
        词法分析器 {
            源码: 源码.chars().collect(),
            当前位置: 0, 行号: 1, 列号: 1,
            文件名: 文件名.to_string(),
            关键字表: 关键字表(),
        }
    }

    pub fn 分析(源码: &str, 文件名: &str) -> Result<Vec<记号>, 词法错误> {
        let mut 分析器 = 词法分析器::新建(源码, 文件名);
        let mut 列表 = Vec::new();
        loop {
            let 记号 = 分析器.下一个记号()?;
            let 结束 = 记号.种类 == 记号种类::文件结束;
            列表.push(记号);
            if 结束 { break; }
        }
        Ok(列表)
    }

    fn 当前位置信息(&self) -> 源位置 {
        源位置::新建(&self.文件名, self.行号, self.列号)
    }

    fn 当前字符(&self) -> Option<char> { self.源码.get(self.当前位置).copied() }

    fn 前进(&mut self) -> Option<char> {
        let 字符 = self.源码.get(self.当前位置).copied();
        if let Some(字) = 字符 {
            self.当前位置 += 1;
            if 字 == '\n' { self.行号 += 1; self.列号 = 1; } else { self.列号 += 1; }
        }
        字符
    }

    fn 查看下一个(&self) -> Option<char> { self.源码.get(self.当前位置 + 1).copied() }

    fn 跳过空白和注释(&mut self) {
        while let Some(字符) = self.当前字符() {
            match 字符 {
                ' ' | '\t' | '\r' | '\n' => { self.前进(); }
                '/' if self.查看下一个() == Some('/') => {
                    while let Some(字) = self.当前字符() { if 字 == '\n' { break; } self.前进(); }
                }
                '/' if self.查看下一个() == Some('*') => {
                    self.前进(); self.前进();
                    while let Some(字) = self.当前字符() {
                        if 字 == '*' && self.查看下一个() == Some('/') { self.前进(); self.前进(); break; }
                        self.前进();
                    }
                }
                _ => break,
            }
        }
    }

    fn 下一个记号(&mut self) -> Result<记号, 词法错误> {
        self.跳过空白和注释();
        let 位置 = self.当前位置信息();
        let 字符 = match self.当前字符() { Some(字) => 字, None => return Ok(记号::新建(记号种类::文件结束, 位置)) };

        match 字符 {
            _ if 是标识符首字符(字符) => self.读取标识符(位置),
            '0'..='9' => self.读取数字(位置),
            '"' => self.读取字符串(位置, '"'),
            '\'' => self.读取字符串(位置, '\''),
            '+' => { self.前进(); if self.当前字符() == Some('=') { self.前进(); Ok(记号::新建(记号种类::加赋值, 位置)) } else { Ok(记号::新建(记号种类::加, 位置)) } }
            '-' => { self.前进(); if self.当前字符() == Some('>') { self.前进(); Ok(记号::新建(记号种类::箭头, 位置)) } else if self.当前字符() == Some('=') { self.前进(); Ok(记号::新建(记号种类::减赋值, 位置)) } else { Ok(记号::新建(记号种类::减, 位置)) } }
            '*' => { self.前进(); if self.当前字符() == Some('=') { self.前进(); Ok(记号::新建(记号种类::乘赋值, 位置)) } else { Ok(记号::新建(记号种类::乘, 位置)) } }
            '/' => { self.前进(); if self.当前字符() == Some('=') { self.前进(); Ok(记号::新建(记号种类::除赋值, 位置)) } else { Ok(记号::新建(记号种类::除, 位置)) } }
            '%' => { self.前进(); Ok(记号::新建(记号种类::取余, 位置)) }
            '=' => { self.前进(); if self.当前字符() == Some('=') { self.前进(); Ok(记号::新建(记号种类::等于, 位置)) } else { Ok(记号::新建(记号种类::赋值, 位置)) } }
            '!' => { self.前进(); if self.当前字符() == Some('=') { self.前进(); Ok(记号::新建(记号种类::不等于, 位置)) } else { Err(词法错误::非法字符('!', 位置)) } }
            '<' => { self.前进(); if self.当前字符() == Some('=') { self.前进(); Ok(记号::新建(记号种类::小于等于, 位置)) } else { Ok(记号::新建(记号种类::小于, 位置)) } }
            '>' => { self.前进(); if self.当前字符() == Some('=') { self.前进(); Ok(记号::新建(记号种类::大于等于, 位置)) } else { Ok(记号::新建(记号种类::大于, 位置)) } }
            '&' => { self.前进(); if self.当前字符() == Some('&') { self.前进(); } Ok(记号::新建(记号种类::且, 位置)) }
            '|' => { self.前进(); if self.当前字符() == Some('>') { self.前进(); Ok(记号::新建(记号种类::管道, 位置)) } else if self.当前字符() == Some('|') { self.前进(); Ok(记号::新建(记号种类::或, 位置)) } else { Ok(记号::新建(记号种类::或, 位置)) } }
            '(' => { self.前进(); Ok(记号::新建(记号种类::左圆括号, 位置)) }
            ')' => { self.前进(); Ok(记号::新建(记号种类::右圆括号, 位置)) }
            '{' => { self.前进(); Ok(记号::新建(记号种类::左花括号, 位置)) }
            '}' => { self.前进(); Ok(记号::新建(记号种类::右花括号, 位置)) }
            '[' => { self.前进(); Ok(记号::新建(记号种类::左方括号, 位置)) }
            ']' => { self.前进(); Ok(记号::新建(记号种类::右方括号, 位置)) }
            '.' => { self.前进(); Ok(记号::新建(记号种类::点, 位置)) }
            ',' => { self.前进(); Ok(记号::新建(记号种类::逗号, 位置)) }
            ':' => { self.前进(); Ok(记号::新建(记号种类::冒号, 位置)) }
            ';' => { self.前进(); Ok(记号::新建(记号种类::分号, 位置)) }
            _ => { self.前进(); Err(词法错误::非法字符(字符, 位置)) }
        }
    }

    fn 读取标识符(&mut self, 位置: 源位置) -> Result<记号, 词法错误> {
        let mut 标识符 = String::new();
        while let Some(字符) = self.当前字符() {
            if 是标识符字符(字符) { 标识符.push(字符); self.前进(); } else { break; }
        }
        if let Some(种类) = self.关键字表.get(&标识符) {
            Ok(记号::新建(种类.clone(), 位置))
        } else {
            Ok(记号::新建(记号种类::标识符(标识符), 位置))
        }
    }

    fn 读取数字(&mut self, 位置: 源位置) -> Result<记号, 词法错误> {
        let mut 数字串 = String::new();
        let mut 是否小数 = false;
        while let Some(字符) = self.当前字符() {
            match 字符 {
                '0'..='9' => { 数字串.push(字符); self.前进(); }
                '.' if !是否小数 && self.查看下一个().map_or(false, |c| c.is_ascii_digit()) => {
                    是否小数 = true; 数字串.push(字符); self.前进();
                }
                _ => break,
            }
        }
        if 是否小数 {
            数字串.parse::<f64>().map(|v| 记号::新建(记号种类::小数字面量(v), 位置.clone())).map_err(|_| 词法错误::非法数字(位置))
        } else {
            数字串.parse::<i64>().map(|v| 记号::新建(记号种类::整数字面量(v), 位置.clone())).map_err(|_| 词法错误::非法数字(位置))
        }
    }

    fn 读取字符串(&mut self, 位置: 源位置, 引号: char) -> Result<记号, 词法错误> {
        self.前进();
        let mut 内容 = String::new();
        while let Some(字符) = self.当前字符() {
            if 字符 == 引号 { self.前进(); return Ok(记号::新建(记号种类::文本字面量(内容), 位置)); }
            if 字符 == '\\' {
                self.前进();
                if let Some(转义) = self.前进() {
                    match 转义 { 'n' => 内容.push('\n'), 't' => 内容.push('\t'), '\\' => 内容.push('\\'), _ => { 内容.push('\\'); 内容.push(转义); } }
                }
            } else { 内容.push(字符); self.前进(); }
        }
        Err(词法错误::未闭合字符串(位置))
    }
}
