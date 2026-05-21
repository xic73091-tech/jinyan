/// 虚拟机 —— 诚言编程语言

use crate::字节码::*;
use crate::值类型::*;
use crate::内置函数;
use crate::错误::运行时错误;
use std::collections::HashMap as 哈希映射;

#[derive(Debug, Clone)]
struct 栈帧 { 返回地址: usize, 局部变量起始: usize, 函数名: String }

pub struct 虚拟机 {
    操作数栈: Vec<值>,
    调用栈: Vec<栈帧>,
    全局变量: 哈希映射<String, 值>,
    指令指针: usize,
    栈帧起始: usize,
}

impl 虚拟机 {
    pub fn 新建() -> Self {
        let mut 虚拟机 = 虚拟机 {
            操作数栈: Vec::with_capacity(256),
            调用栈: Vec::with_capacity(64),
            全局变量: 哈希映射::new(),
            指令指针: 0, 栈帧起始: 0,
        };
        内置函数::注册内置函数(&mut 虚拟机.全局变量);
        虚拟机
    }

    pub fn 执行(&mut self, 代码块: &字节码块) -> Result<(), 运行时错误> {
        self.指令指针 = 0;
        loop {
            if self.指令指针 >= 代码块.指令.len() { break; }
            if self.操作数栈.len() > 10000 { return Err(运行时错误::栈溢出); }
            let 操作码 = self.读取字节(代码块)?;
            match 操作码 {
                x if x == 操作码::压入常量 as u8 => { let i = self.读取u16(代码块)? as usize; if i >= 代码块.常量池.len() { return Err(运行时错误::类型错误(格式化!("常量池索引越界: {}", i))); } self.操作数栈.push(self.常量转值(&代码块.常量池[i])); }
                x if x == 操作码::压入真 as u8 => { self.操作数栈.push(值::布尔(true)); }
                x if x == 操作码::压入假 as u8 => { self.操作数栈.push(值::布尔(false)); }
                x if x == 操作码::压入空 as u8 => { self.操作数栈.push(值::空值); }
                x if x == 操作码::弹出 as u8 => { self.操作数栈.pop(); }
                x if x == 操作码::复制 as u8 => { let v = self.操作数栈.last().cloned().ok_or(运行时错误::空引用)?; self.操作数栈.push(v); }
                x if x == 操作码::读取局部 as u8 => { let s = self.读取字节(代码块)? as usize; let v = self.操作数栈.get(self.栈帧起始 + s).cloned().ok_or(运行时错误::空引用)?; self.操作数栈.push(v); }
                x if x == 操作码::设置局部 as u8 => { let s = self.读取字节(代码块)? as usize; let v = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let i = self.栈帧起始 + s; if i >= self.操作数栈.len() { self.操作数栈.resize(i + 1, 值::空值); } self.操作数栈[i] = v; }
                x if x == 操作码::读取全局 as u8 => { let i = self.读取u16(代码块)? as usize; if i >= 代码块.常量池.len() { return Err(运行时错误::类型错误(格式化!("常量池索引越界: {}", i))); } let 名 = match &代码块.常量池[i] { 常量::文本(s) => s.clone(), _ => return Err(运行时错误::类型错误("全局变量名必须是文本".into())) }; let v = self.全局变量.get(&名).cloned().unwrap_or(值::空值); self.操作数栈.push(v); }
                x if x == 操作码::设置全局 as u8 => { let i = self.读取u16(代码块)? as usize; if i >= 代码块.常量池.len() { return Err(运行时错误::类型错误(格式化!("常量池索引越界: {}", i))); } let 名 = match &代码块.常量池[i] { 常量::文本(s) => s.clone(), _ => return Err(运行时错误::类型错误("全局变量名必须是文本".into())) }; let v = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.全局变量.insert(名, v); }
                x if x == 操作码::加 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(l.加(&r)?); }
                x if x == 操作码::减 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(l.减(&r)?); }
                x if x == 操作码::乘 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(l.乘(&r)?); }
                x if x == 操作码::除 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(l.除(&r)?); }
                x if x == 操作码::取余 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(l.取余(&r)?); }
                x if x == 操作码::取反 as u8 => { let v = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(v.取反()?); }
                x if x == 操作码::相等 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(值::布尔(l.相等(&r))); }
                x if x == 操作码::不等 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(值::布尔(!l.相等(&r))); }
                x if x == 操作码::小于 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(值::布尔(l.小于(&r)?)); }
                x if x == 操作码::大于 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(值::布尔(r.小于(&l)?)); }
                x if x == 操作码::小于等于 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(值::布尔(l.小于(&r)? || l.相等(&r))); }
                x if x == 操作码::大于等于 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(值::布尔(r.小于(&l)? || l.相等(&r))); }
                x if x == 操作码::逻辑与 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(值::布尔(l.转布尔() && r.转布尔())); }
                x if x == 操作码::逻辑或 as u8 => { let r = self.操作数栈.pop().ok_or(运行时错误::空引用)?; let l = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(值::布尔(l.转布尔() || r.转布尔())); }
                x if x == 操作码::逻辑非 as u8 => { let v = self.操作数栈.pop().ok_or(运行时错误::空引用)?; self.操作数栈.push(值::布尔(!v.转布尔())); }
                x if x == 操作码::无条件跳转 as u8 => { let 偏移 = self.读取u16(代码块)? as i16; let 目标 = (self.指令指针 as i64 + 偏移 as i64) as usize; if 目标 >= 代码块.指令.len() { return Err(运行时错误::类型错误(格式化!("跳转目标越界: {}", 目标))); } self.指令指针 = 目标; }
                x if x == 操作码::条件跳转 as u8 => { let 偏移 = self.读取u16(代码块)? as i16; let 条件 = self.操作数栈.pop().ok_or(运行时错误::空引用)?; if 条件.转布尔() { let 目标 = (self.指令指针 as i64 + 偏移 as i64) as usize; if 目标 >= 代码块.指令.len() { return Err(运行时错误::类型错误(格式化!("跳转目标越界: {}", 目标))); } self.指令指针 = 目标; } }
                x if x == 操作码::假性跳转 as u8 => { let 偏移 = self.读取u16(代码块)? as i16; let 条件 = self.操作数栈.last().cloned().ok_or(运行时错误::空引用)?; if !条件.转布尔() { self.操作数栈.pop(); let 目标 = (self.指令指针 as i64 + 偏移 as i64) as usize; if 目标 >= 代码块.指令.len() { return Err(运行时错误::类型错误(格式化!("跳转目标越界: {}", 目标))); } self.指令指针 = 目标; } else { self.操作数栈.pop(); } }
                x if x == 操作码::调用 as u8 => {
                    let 参数数量 = self.读取字节(代码块)? as usize;
                    if self.操作数栈.len() < 参数数量 + 1 { return Err(运行时错误::类型错误(格式化!("调用栈下溢: 需要 {} 个参数，栈中只有 {} 个值", 参数数量, self.操作数栈.len()))); }
                    let 函数位置 = self.操作数栈.len() - 参数数量 - 1;
                    let 函数值 = self.操作数栈[函数位置].clone();
                    match 函数值 {
                        值::函数(函数对象) => {
                            if self.调用栈.len() >= 256 { return Err(运行时错误::栈溢出); }
                            self.调用栈.push(栈帧 { 返回地址: self.指令指针, 局部变量起始: self.栈帧起始, 函数名: 函数对象.名称.clone() });
                            self.栈帧起始 = 函数位置;
                            self.指令指针 = 函数对象.起始地址;
                            let 需要 = 函数位置 + 函数对象.局部变量数量 as usize;
                            if self.操作数栈.len() < 需要 { self.操作数栈.resize(需要, 值::空值); }
                        }
                        值::原生函数(原生函数对象) => {
                            let mut 参数 = Vec::new();
                            for _ in 0..参数数量 { 参数.push(self.操作数栈.pop().ok_or(运行时错误::空引用)?); }
                            参数.reverse();
                            self.操作数栈.pop();
                            let 结果 = (原生函数对象.函数)(&参数);
                            self.操作数栈.push(结果);
                        }
                        _ => return Err(运行时错误::类型错误(格式化!("不能调用非函数值"))),
                    }
                }
                x if x == 操作码::返回 as u8 => {
                    let 返回值 = self.操作数栈.pop().unwrap_or(值::空值);
                    if let Some(帧) = self.调用栈.pop() {
                        self.操作数栈.truncate(self.栈帧起始);
                        self.栈帧起始 = 帧.局部变量起始;
                        self.指令指针 = 帧.返回地址;
                        self.操作数栈.push(返回值);
                    } else { break; }
                }
                x if x == 操作码::构建数组 as u8 => {
                    let 大小 = self.读取u16(代码块)? as usize;
                    let mut 元素 = Vec::with_capacity(大小);
                    for _ in 0..大小 { 元素.push(self.操作数栈.pop().ok_or(运行时错误::空引用)?); }
                    元素.reverse();
                    self.操作数栈.push(值::数组(元素));
                }
                x if x == 操作码::索引读取 as u8 => {
                    let 索引 = self.操作数栈.pop().ok_or(运行时错误::空引用)?;
                    let 对象 = self.操作数栈.pop().ok_or(运行时错误::空引用)?;
                    match (&对象, &索引) {
                        (值::数组(a), 值::整数(i)) => { let idx = *i as usize; if idx >= a.len() { return Err(运行时错误::索引越界(idx, a.len())); } self.操作数栈.push(a[idx].clone()); }
                        (值::映射(m), 值::文本(k)) => { self.操作数栈.push(m.get(k).cloned().unwrap_or(值::空值)); }
                        _ => return Err(运行时错误::类型错误(格式化!("不能对 {} 进行索引访问", 对象.类型名()))),
                    }
                }
                x if x == 操作码::索引设置 as u8 => {
                    let 新值 = self.操作数栈.pop().ok_or(运行时错误::空引用)?;
                    let 索引 = self.操作数栈.pop().ok_or(运行时错误::空引用)?;
                    let 对象 = self.操作数栈.pop().ok_or(运行时错误::空引用)?;
                    match (&对象, &索引) {
                        (值::数组(a), 值::整数(i)) => {
                            let idx = *i as usize;
                            if idx >= a.len() { return Err(运行时错误::索引越界(idx, a.len())); }
                            let mut 新数组 = a.clone();
                            新数组[idx] = 新值;
                            self.操作数栈.push(值::数组(新数组));
                        }
                        (值::映射(m), 值::文本(k)) => {
                            let mut 新映射 = m.clone();
                            新映射.insert(k.clone(), 新值);
                            self.操作数栈.push(值::映射(新映射));
                        }
                        _ => return Err(运行时错误::类型错误(格式化!("不能对 {} 进行索引设置", 对象.类型名()))),
                    }
                }
                x if x == 操作码::属性读取 as u8 => {
                    let i = self.读取u16(代码块)? as usize;
                    if i >= 代码块.常量池.len() { return Err(运行时错误::类型错误(格式化!("常量池索引越界: {}", i))); }
                    let 属性名 = match &代码块.常量池[i] { 常量::文本(s) => s.clone(), _ => return Err(运行时错误::类型错误("属性名必须是文本".into())) };
                    let 对象 = self.操作数栈.pop().ok_or(运行时错误::空引用)?;
                    let 结果 = match &对象 {
                        值::映射(m) => m.get(&属性名).cloned().unwrap_or(值::空值),
                        值::实例(实例) => 实例.属性.get(&属性名).cloned().unwrap_or(值::空值),
                        _ => return Err(运行时错误::类型错误(格式化!("不能对 {} 进行属性访问", 对象.类型名()))),
                    };
                    self.操作数栈.push(结果);
                }
                x if x == 操作码::属性设置 as u8 => {
                    let i = self.读取u16(代码块)? as usize;
                    if i >= 代码块.常量池.len() { return Err(运行时错误::类型错误(格式化!("常量池索引越界: {}", i))); }
                    let 属性名 = match &代码块.常量池[i] { 常量::文本(s) => s.clone(), _ => return Err(运行时错误::类型错误("属性名必须是文本".into())) };
                    let 新值 = self.操作数栈.pop().ok_or(运行时错误::空引用)?;
                    let 对象 = self.操作数栈.pop().ok_or(运行时错误::空引用)?;
                    match &对象 {
                        值::映射(m) => {
                            let mut 新映射 = m.clone();
                            新映射.insert(属性名, 新值);
                            self.操作数栈.push(值::映射(新映射));
                        }
                        值::实例(实例) => {
                            let mut 新实例 = 实例.clone();
                            新实例.属性.insert(属性名.clone(), 新值);
                            self.操作数栈.push(值::实例(新实例));
                        }
                        _ => return Err(运行时错误::类型错误(格式化!("不能对 {} 进行属性设置", 对象.类型名()))),
                    }
                }
                x if x == 操作码::构建映射 as u8 => {
                    let 大小 = self.读取u16(代码块)? as usize;
                    let mut 映射 = std::collections::HashMap::new();
                    for _ in 0..大小 {
                        let 值_v = self.操作数栈.pop().ok_or(运行时错误::空引用)?;
                        let 键 = self.操作数栈.pop().ok_or(运行时错误::空引用)?;
                        let 键名 = match 键 { 值::文本(s) => s, _ => return Err(运行时错误::类型错误("映射键必须是文本".into())) };
                        映射.insert(键名, 值_v);
                    }
                    self.操作数栈.push(值::映射(映射));
                }
                x if x == 操作码::构建实例 as u8 => {
                    let i = self.读取u16(代码块)? as usize;
                    if i >= 代码块.常量池.len() { return Err(运行时错误::类型错误(格式化!("常量池索引越界: {}", i))); }
                    let 类名 = match &代码块.常量池[i] { 常量::文本(s) => s.clone(), _ => return Err(运行时错误::类型错误("类名必须是文本".into())) };
                    self.操作数栈.push(值::实例(实例对象 { 类名, 属性: std::collections::HashMap::new() }));
                }
                x if x == 操作码::打印 as u8 => { let v = self.操作数栈.pop().ok_or(运行时错误::空引用)?; 打印行!("{}", v.转文本()); }
                x if x == 操作码::停止 as u8 => { break; }
                _ => return Err(运行时错误::类型错误(格式化!("未知操作码: {}", 操作码))),
            }
        }
        Ok(())
    }

    fn 读取字节(&mut self, 代码块: &字节码块) -> Result<u8, 运行时错误> {
        if self.指令指针 >= 代码块.指令.len() { return Err(运行时错误::类型错误("指令指针越界: 字节码不完整".into())); }
        let b = 代码块.指令[self.指令指针]; self.指令指针 += 1; Ok(b)
    }
    fn 读取u16(&mut self, 代码块: &字节码块) -> Result<u16, 运行时错误> { let 高 = self.读取字节(代码块)? as u16; let 低 = self.读取字节(代码块)? as u16; Ok((高 << 8) | 低) }

    fn 常量转值(&self, 常量: &常量) -> 值 {
        match 常量 {
            常量::整数(n) => 值::整数(*n),
            常量::小数(n) => 值::小数(*n),
            常量::文本(s) => 值::文本(s.clone()),
            常量::函数(信息) => 值::函数(函数对象 { 名称: 信息.名称.clone(), 参数数量: 信息.参数数量, 局部变量数量: 信息.局部变量数量, 起始地址: 信息.起始地址 }),
        }
    }
}
