/// 字节码指令集 —— 诚言编程语言

use std::fmt;

/// 操作码
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum 操作码 {
    压入常量 = 0, 压入真, 压入假, 压入空, 弹出, 复制,
    读取局部, 设置局部, 读取全局, 设置全局,
    加, 减, 乘, 除, 取余, 取反,
    相等, 不等, 小于, 大于, 小于等于, 大于等于,
    逻辑与, 逻辑或, 逻辑非,
    无条件跳转, 条件跳转, 假性跳转,
    调用, 返回,
    构建数组, 构建映射, 构建实例, 索引读取, 索引设置, 属性读取, 属性设置,
    打印, 停止,
}

impl 操作码 {
    pub fn 操作数大小(&self) -> usize {
        match self {
            操作码::压入常量 | 操作码::读取全局 | 操作码::设置全局
            | 操作码::无条件跳转 | 操作码::条件跳转 | 操作码::假性跳转
            | 操作码::构建数组 | 操作码::构建映射 | 操作码::构建实例 => 2,
            操作码::读取局部 | 操作码::设置局部 | 操作码::调用 => 1,
            _ => 0,
        }
    }
}

/// 常量池条目
#[derive(Debug, Clone)]
pub enum 常量 {
    整数(i64),
    小数(f64),
    文本(String),
    函数(函数信息),
}

#[derive(Debug, Clone)]
pub struct 函数信息 {
    pub 名称: String,
    pub 参数数量: u8,
    pub 局部变量数量: u8,
    pub 起始地址: usize,
}

/// 字节码块
#[derive(Debug)]
pub struct 字节码块 {
    pub 指令: Vec<u8>,
    pub 常量池: Vec<常量>,
    pub 行号表: Vec<(usize, usize)>,
}

impl 字节码块 {
    pub fn 新建() -> Self {
        字节码块 { 指令: Vec::new(), 常量池: Vec::new(), 行号表: Vec::new() }
    }

    pub fn 写入操作码(&mut self, 操作码: 操作码, 行号: usize) {
        self.行号表.push((self.指令.len(), 行号));
        self.指令.push(操作码 as u8);
    }

    pub fn 写入字节(&mut self, 字节: u8) {
        self.指令.push(字节);
    }

    pub fn 写入u16(&mut self, 值: u16) {
        self.指令.push((值 >> 8) as u8);
        self.指令.push((值 & 0xFF) as u8);
    }

    pub fn 添加常量(&mut self, 常量: 常量) -> u16 {
        let 索引 = self.常量池.len() as u16;
        self.常量池.push(常量);
        索引
    }

    pub fn 当前地址(&self) -> usize {
        self.指令.len()
    }

    pub fn 回填跳转(&mut self, 地址: usize, 目标: usize) {
        let 偏移 = (目标 as i64 - 地址 as i64 - 2) as i16 as u16;
        self.指令[地址] = (偏移 >> 8) as u8;
        self.指令[地址 + 1] = (偏移 & 0xFF) as u8;
    }

    pub fn 反汇编(&self) -> String {
        let mut 输出 = String::new();
        输出.push_str("=== 常量池 ===\n");
        for (i, 常量) in self.常量池.iter().enumerate() {
            输出.push_str(&格式化!("[{}] {:?}\n", i, 常量));
        }
        输出.push_str("\n=== 指令 ===\n");
        let mut i = 0;
        while i < self.指令.len() {
            输出.push_str(&格式化!("{:04} ", i));
            let 操作码值 = self.指令[i];
            match 操作码值 {
                x if x == 操作码::压入常量 as u8 => {
                    let idx = ((self.指令[i+1] as u16) << 8) | self.指令[i+2] as u16;
                    输出.push_str(&格式化!("压入常量 {}\n", idx));
                    i += 3;
                }
                x if x == 操作码::读取局部 as u8 => {
                    输出.push_str(&格式化!("读取局部 {}\n", self.指令[i+1]));
                    i += 2;
                }
                x if x == 操作码::设置局部 as u8 => {
                    输出.push_str(&格式化!("设置局部 {}\n", self.指令[i+1]));
                    i += 2;
                }
                x if x == 操作码::调用 as u8 => {
                    输出.push_str(&格式化!("调用 {}\n", self.指令[i+1]));
                    i += 2;
                }
                x if x == 操作码::无条件跳转 as u8 => {
                    let 偏移 = ((self.指令[i+1] as u16) << 8) | self.指令[i+2] as u16;
                    输出.push_str(&格式化!("无条件跳转 {}\n", i + 3 + 偏移 as usize));
                    i += 3;
                }
                x if x == 操作码::假性跳转 as u8 => {
                    let 偏移 = ((self.指令[i+1] as u16) << 8) | self.指令[i+2] as u16;
                    输出.push_str(&格式化!("假性跳转 {}\n", i + 3 + 偏移 as usize));
                    i += 3;
                }
                _ => {
                    let 操作数大小 = match 操作码值 {
                        x if x == 操作码::读取全局 as u8 || x == 操作码::设置全局 as u8
                        || x == 操作码::无条件跳转 as u8 || x == 操作码::条件跳转 as u8
                        || x == 操作码::假性跳转 as u8 || x == 操作码::构建数组 as u8
                        || x == 操作码::构建映射 as u8 => 2,
                        x if x == 操作码::读取局部 as u8 || x == 操作码::设置局部 as u8
                        || x == 操作码::调用 as u8 => 1,
                        _ => 0,
                    };
                    输出.push_str(&格式化!("操作码({})\n", 操作码值));
                    i += 1 + 操作数大小;
                }
            }
        }
        输出
    }
}
