/// 语法分析器 —— 诚言编程语言

use crate::记号::*;
use crate::抽象语法树::*;
use crate::错误::语法错误;

pub struct 语法分析器 {
    记号列表: Vec<记号>,
    当前位置: usize,
    结束哨兵: 记号,
}

impl 语法分析器 {
    pub fn 新建(记号列表: Vec<记号>) -> Self {
        语法分析器 {
            记号列表,
            当前位置: 0,
            结束哨兵: 记号::新建(记号种类::文件结束, 源位置::新建("", 0, 0)),
        }
    }

    pub fn 解析(记号列表: &[记号]) -> Result<程序, 语法错误> {
        let mut 解析器 = 语法分析器::新建(记号列表.to_vec());
        let mut 程序 = 程序::新建();
        while !解析器.是结束() {
            程序.语句列表.push(解析器.解析语句()?);
        }
        Ok(程序)
    }

    fn 当前记号(&self) -> &记号 { self.记号列表.get(self.当前位置).unwrap_or(&self.结束哨兵) }
    fn 前进(&mut self) -> 记号 { let t = self.记号列表.get(self.当前位置).cloned().unwrap_or_else(|| self.结束哨兵.clone()); self.当前位置 += 1; t }
    fn 检查(&self, 种类: &记号种类) -> bool { std::mem::discriminant(&self.当前记号().种类) == std::mem::discriminant(种类) }
    fn 是结束(&self) -> bool { self.当前记号().种类 == 记号种类::文件结束 }

    fn 匹配记号(&mut self, 期望: &记号种类) -> Result<记号, 语法错误> {
        if std::mem::discriminant(&self.当前记号().种类) == std::mem::discriminant(期望) {
            Ok(self.前进())
        } else {
            Err(语法错误::意外记号 { 期望: 格式化!("{:?}", 期望), 实际: 格式化!("{:?}", self.当前记号().种类), 位置: self.当前记号().位置.clone() })
        }
    }

    fn 解析语句(&mut self) -> Result<语句, 语法错误> {
        match &self.当前记号().种类 {
            记号种类::让 => self.解析变量声明(false),
            记号种类::定义 => self.解析函数定义(),
            记号种类::如果 => self.解析如果(),
            记号种类::当 => self.解析当循环(),
            记号种类::对 => self.解析遍历(),
            记号种类::类 => self.解析类定义(),
            记号种类::返回 => { self.前进(); if self.检查(&记号种类::右花括号) || self.是结束() { Ok(语句::返回(None)) } else { Ok(语句::返回(Some(self.解析表达式()?))) } }
            记号种类::跳出 => { self.前进(); Ok(语句::跳出) }
            记号种类::继续 => { self.前进(); Ok(语句::继续) }
            记号种类::尝试 => self.解析尝试捕获(),
            记号种类::抛出 => { self.前进(); Ok(语句::抛出(self.解析表达式()?)) }
            _ => self.解析表达式语句(),
        }
    }

    fn 解析变量声明(&mut self, 是否常量: bool) -> Result<语句, 语法错误> {
        self.前进();
        let 名称 = match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => return Err(语法错误::意外记号 { 期望: "标识符".into(), 实际: 格式化!("{:?}", t), 位置: self.当前记号().位置.clone() }) };
        let mut 初始值 = None;
        if self.检查(&记号种类::赋值) { self.前进(); 初始值 = Some(self.解析表达式()?); }
        Ok(语句::变量声明 { 名称, 初始值, 是否常量, 类型标注: None })
    }

    fn 解析如果(&mut self) -> Result<语句, 语法错误> {
        self.前进();
        let 条件 = self.解析表达式()?;
        self.匹配记号(&记号种类::左花括号)?;
        let 真分支 = self.解析语句块()?;
        self.匹配记号(&记号种类::右花括号)?;
        let 假分支 = if self.检查(&记号种类::否则如果) { Some(vec![self.解析如果()?]) }
            else if self.检查(&记号种类::否则) { self.前进(); self.匹配记号(&记号种类::左花括号)?; let 块 = self.解析语句块()?; self.匹配记号(&记号种类::右花括号)?; Some(块) }
            else { None };
        Ok(语句::如果 { 条件, 真分支, 假分支 })
    }

    fn 解析当循环(&mut self) -> Result<语句, 语法错误> {
        self.前进();
        let 条件 = self.解析表达式()?;
        self.匹配记号(&记号种类::左花括号)?;
        let 体 = self.解析语句块()?;
        self.匹配记号(&记号种类::右花括号)?;
        Ok(语句::当循环 { 条件, 体 })
    }

    fn 解析遍历(&mut self) -> Result<语句, 语法错误> {
        self.前进(); // 对
        let 变量名 = match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), 记号种类::每项 => match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => return Err(语法错误::意外记号 { 期望: "标识符".into(), 实际: 格式化!("{:?}", t), 位置: self.当前记号().位置.clone() }) }, t => return Err(语法错误::意外记号 { 期望: "每项".into(), 实际: 格式化!("{:?}", t), 位置: self.当前记号().位置.clone() }) };
        self.匹配记号(&记号种类::在)?;
        let 集合 = self.解析表达式()?;
        if self.检查(&记号种类::中) { self.前进(); }
        self.匹配记号(&记号种类::左花括号)?;
        let 体 = self.解析语句块()?;
        self.匹配记号(&记号种类::右花括号)?;
        Ok(语句::遍历 { 变量名, 集合, 体 })
    }

    fn 解析函数定义(&mut self) -> Result<语句, 语法错误> {
        self.前进(); // 定义
        let 名称 = match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => return Err(语法错误::意外记号 { 期望: "函数名".into(), 实际: 格式化!("{:?}", t), 位置: self.当前记号().位置.clone() }) };
        self.匹配记号(&记号种类::左圆括号)?;
        let 参数 = self.解析参数列表()?;
        self.匹配记号(&记号种类::右圆括号)?;
        let mut 返回类型 = None;
        if self.检查(&记号种类::箭头) { self.前进(); 返回类型 = Some(match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => 格式化!("{:?}", t) }); }
        self.匹配记号(&记号种类::左花括号)?;
        let 体 = self.解析语句块()?;
        self.匹配记号(&记号种类::右花括号)?;
        Ok(语句::函数定义 { 名称, 参数, 返回类型, 体, 是否构造: false })
    }

    fn 解析参数列表(&mut self) -> Result<Vec<参数定义>, 语法错误> {
        let mut 参数 = Vec::new();
        if !self.检查(&记号种类::右圆括号) {
            参数.push(self.解析单个参数()?);
            while self.检查(&记号种类::逗号) { self.前进(); 参数.push(self.解析单个参数()?); }
        }
        Ok(参数)
    }

    fn 解析单个参数(&mut self) -> Result<参数定义, 语法错误> {
        let 名称 = match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => return Err(语法错误::意外记号 { 期望: "参数名".into(), 实际: 格式化!("{:?}", t), 位置: self.当前记号().位置.clone() }) };
        let mut 类型标注 = None;
        if self.检查(&记号种类::冒号) { self.前进(); 类型标注 = Some(match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => 格式化!("{:?}", t) }); }
        Ok(参数定义 { 名称, 类型标注 })
    }

    fn 解析类定义(&mut self) -> Result<语句, 语法错误> {
        self.前进(); // 类
        let 名称 = match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => return Err(语法错误::意外记号 { 期望: "类名".into(), 实际: 格式化!("{:?}", t), 位置: self.当前记号().位置.clone() }) };
        let mut 父类 = None;
        if self.检查(&记号种类::继承) { self.前进(); 父类 = Some(match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => 格式化!("{:?}", t) }); }
        self.匹配记号(&记号种类::左花括号)?;
        let mut 成员 = Vec::new();
        while !self.检查(&记号种类::右花括号) && !self.是结束() { 成员.push(self.解析类成员()?); }
        self.匹配记号(&记号种类::右花括号)?;
        Ok(语句::类定义 { 名称, 父类, 接口列表: Vec::new(), 成员 })
    }

    fn 解析类成员(&mut self) -> Result<类成员, 语法错误> {
        let mut 是否公有 = true;
        if self.检查(&记号种类::公有) { self.前进(); 是否公有 = true; }
        if self.检查(&记号种类::私有) { self.前进(); 是否公有 = false; }
        if self.检查(&记号种类::构造) {
            self.前进();
            self.匹配记号(&记号种类::左圆括号)?;
            let 参数 = self.解析参数列表()?;
            self.匹配记号(&记号种类::右圆括号)?;
            self.匹配记号(&记号种类::左花括号)?;
            let 体 = self.解析语句块()?;
            self.匹配记号(&记号种类::右花括号)?;
            return Ok(类成员::构造函数 { 参数, 体 });
        }
        if self.检查(&记号种类::方法) {
            self.前进();
            let 名称 = match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => return Err(语法错误::意外记号 { 期望: "方法名".into(), 实际: 格式化!("{:?}", t), 位置: self.当前记号().位置.clone() }) };
            self.匹配记号(&记号种类::左圆括号)?;
            let 参数 = self.解析参数列表()?;
            self.匹配记号(&记号种类::右圆括号)?;
            let mut 返回类型 = None;
            if self.检查(&记号种类::箭头) { self.前进(); 返回类型 = Some(match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => 格式化!("{:?}", t) }); }
            self.匹配记号(&记号种类::左花括号)?;
            let 体 = self.解析语句块()?;
            self.匹配记号(&记号种类::右花括号)?;
            return Ok(类成员::方法 { 名称, 参数, 返回类型, 体, 是否公有, 是否覆盖: false });
        }
        // 属性
        let 名称 = match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => return Err(语法错误::意外记号 { 期望: "属性名".into(), 实际: 格式化!("{:?}", t), 位置: self.当前记号().位置.clone() }) };
        let mut 类型标注 = None;
        if self.检查(&记号种类::冒号) { self.前进(); 类型标注 = Some(match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => 格式化!("{:?}", t) }); }
        Ok(类成员::属性 { 名称, 类型标注, 是否公有 })
    }

    fn 解析尝试捕获(&mut self) -> Result<语句, 语法错误> {
        self.前进(); // 尝试
        self.匹配记号(&记号种类::左花括号)?;
        let 体 = self.解析语句块()?;
        self.匹配记号(&记号种类::右花括号)?;
        self.匹配记号(&记号种类::捕获)?;
        let 捕获变量 = match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => return Err(语法错误::意外记号 { 期望: "变量名".into(), 实际: 格式化!("{:?}", t), 位置: self.当前记号().位置.clone() }) };
        self.匹配记号(&记号种类::左花括号)?;
        let 捕获体 = self.解析语句块()?;
        self.匹配记号(&记号种类::右花括号)?;
        Ok(语句::尝试捕获 { 体, 捕获变量, 捕获体 })
    }

    fn 解析语句块(&mut self) -> Result<Vec<语句>, 语法错误> {
        let mut 列表 = Vec::new();
        while !self.检查(&记号种类::右花括号) && !self.是结束() { 列表.push(self.解析语句()?); }
        Ok(列表)
    }

    fn 解析表达式语句(&mut self) -> Result<语句, 语法错误> {
        // 检查是否是 let/const 声明
        if self.检查(&记号种类::恒定) { return self.解析变量声明(true); }
        let 表达式 = self.解析表达式()?;
        if self.检查(&记号种类::赋值) { self.前进(); let 值 = self.解析表达式()?; return Ok(语句::赋值 { 目标: 表达式, 值 }); }
        Ok(语句::表达式语句(表达式))
    }

    // 表达式解析 —— 优先级爬升
    fn 解析表达式(&mut self) -> Result<表达式, 语法错误> { self.解析逻辑或() }

    fn 解析逻辑或(&mut self) -> Result<表达式, 语法错误> {
        let mut 左 = self.解析逻辑与()?;
        while self.检查(&记号种类::或) { self.前进(); let 右 = self.解析逻辑与()?; 左 = 表达式::二元运算 { 左: Box::new(左), 运算符: 运算符类型::逻辑或, 右: Box::new(右) }; }
        Ok(左)
    }

    fn 解析逻辑与(&mut self) -> Result<表达式, 语法错误> {
        let mut 左 = self.解析比较()?;
        while self.检查(&记号种类::且) { self.前进(); let 右 = self.解析比较()?; 左 = 表达式::二元运算 { 左: Box::new(左), 运算符: 运算符类型::逻辑与, 右: Box::new(右) }; }
        Ok(左)
    }

    fn 解析比较(&mut self) -> Result<表达式, 语法错误> {
        let mut 左 = self.解析加减()?;
        loop {
            let 运算符 = match &self.当前记号().种类 {
                记号种类::等于 => 运算符类型::等于, 记号种类::不等于 => 运算符类型::不等于,
                记号种类::小于 => 运算符类型::小于, 记号种类::大于 => 运算符类型::大于,
                记号种类::小于等于 => 运算符类型::小于等于, 记号种类::大于等于 => 运算符类型::大于等于,
                _ => break,
            };
            self.前进();
            let 右 = self.解析加减()?;
            左 = 表达式::二元运算 { 左: Box::new(左), 运算符, 右: Box::new(右) };
        }
        Ok(左)
    }

    fn 解析加减(&mut self) -> Result<表达式, 语法错误> {
        let mut 左 = self.解析乘除()?;
        loop {
            match &self.当前记号().种类 {
                记号种类::加 => { self.前进(); let 右 = self.解析乘除()?; 左 = 表达式::二元运算 { 左: Box::new(左), 运算符: 运算符类型::加, 右: Box::new(右) }; }
                记号种类::减 => { self.前进(); let 右 = self.解析乘除()?; 左 = 表达式::二元运算 { 左: Box::new(左), 运算符: 运算符类型::减, 右: Box::new(右) }; }
                _ => break,
            }
        }
        Ok(左)
    }

    fn 解析乘除(&mut self) -> Result<表达式, 语法错误> {
        let mut 左 = self.解析一元()?;
        loop {
            match &self.当前记号().种类 {
                记号种类::乘 => { self.前进(); let 右 = self.解析一元()?; 左 = 表达式::二元运算 { 左: Box::new(左), 运算符: 运算符类型::乘, 右: Box::new(右) }; }
                记号种类::除 => { self.前进(); let 右 = self.解析一元()?; 左 = 表达式::二元运算 { 左: Box::new(左), 运算符: 运算符类型::除, 右: Box::new(右) }; }
                记号种类::取余 => { self.前进(); let 右 = self.解析一元()?; 左 = 表达式::二元运算 { 左: Box::new(左), 运算符: 运算符类型::取余, 右: Box::new(右) }; }
                _ => break,
            }
        }
        Ok(左)
    }

    fn 解析一元(&mut self) -> Result<表达式, 语法错误> {
        match &self.当前记号().种类 {
            记号种类::减 => { self.前进(); Ok(表达式::一元运算 { 运算符: 运算符类型::取反, 操作数: Box::new(self.解析一元()?) }) }
            记号种类::非 => { self.前进(); Ok(表达式::一元运算 { 运算符: 运算符类型::逻辑非, 操作数: Box::new(self.解析一元()?) }) }
            _ => self.解析后缀(),
        }
    }

    fn 解析后缀(&mut self) -> Result<表达式, 语法错误> {
        let mut 表达式 = self.解析原子()?;
        loop {
            match &self.当前记号().种类 {
                记号种类::左圆括号 => {
                    self.前进();
                    let mut 参数 = Vec::new();
                    if !self.检查(&记号种类::右圆括号) { 参数.push(self.解析表达式()?); while self.检查(&记号种类::逗号) { self.前进(); 参数.push(self.解析表达式()?); } }
                    self.匹配记号(&记号种类::右圆括号)?;
                    表达式 = 表达式::函数调用 { 函数: Box::new(表达式), 参数 };
                }
                记号种类::点 => {
                    self.前进();
                    let 成员 = match &self.前进().种类 { 记号种类::标识符(n) => n.clone(), t => return Err(语法错误::意外记号 { 期望: "属性名".into(), 实际: 格式化!("{:?}", t), 位置: self.当前记号().位置.clone() }) };
                    表达式 = 表达式::成员访问 { 对象: Box::new(表达式), 成员 };
                }
                记号种类::左方括号 => {
                    self.前进();
                    let 索引 = self.解析表达式()?;
                    self.匹配记号(&记号种类::右方括号)?;
                    表达式 = 表达式::索引访问 { 对象: Box::new(表达式), 索引: Box::new(索引) };
                }
                _ => break,
            }
        }
        Ok(表达式)
    }

    fn 解析原子(&mut self) -> Result<表达式, 语法错误> {
        match &self.当前记号().种类.clone() {
            记号种类::整数字面量(n) => { let n = *n; self.前进(); Ok(表达式::整数(n)) }
            记号种类::小数字面量(n) => { let n = *n; self.前进(); Ok(表达式::小数(n)) }
            记号种类::文本字面量(s) => { let s = s.clone(); self.前进(); Ok(表达式::文本(s)) }
            记号种类::真 => { self.前进(); Ok(表达式::布尔(true)) }
            记号种类::假 => { self.前进(); Ok(表达式::布尔(false)) }
            记号种类::标识符(n) => { let n = n.clone(); self.前进(); Ok(表达式::标识符(n)) }
            记号种类::本 => { self.前进(); Ok(表达式::本引用) }
            记号种类::左圆括号 => { self.前进(); let 表达式 = self.解析表达式()?; self.匹配记号(&记号种类::右圆括号)?; Ok(表达式) }
            记号种类::左方括号 => {
                self.前进();
                let mut 元素 = Vec::new();
                if !self.检查(&记号种类::右方括号) { 元素.push(self.解析表达式()?); while self.检查(&记号种类::逗号) { self.前进(); 元素.push(self.解析表达式()?); } }
                self.匹配记号(&记号种类::右方括号)?;
                Ok(表达式::数组字面量(元素))
            }
            记号种类::左花括号 => {
                self.前进();
                let mut 对 = Vec::new();
                if !self.检查(&记号种类::右花括号) {
                    let 键 = self.解析表达式()?;
                    self.匹配记号(&记号种类::冒号)?;
                    let 值 = self.解析表达式()?;
                    对.push((键, 值));
                    while self.检查(&记号种类::逗号) {
                        self.前进();
                        let 键 = self.解析表达式()?;
                        self.匹配记号(&记号种类::冒号)?;
                        let 值 = self.解析表达式()?;
                        对.push((键, 值));
                    }
                }
                self.匹配记号(&记号种类::右花括号)?;
                Ok(表达式::映射字面量(对))
            }
            _ => Err(语法错误::无效表达式(self.当前记号().位置.clone())),
        }
    }
}
