/// 垃圾回收器 —— 诚言编程语言
/// 标记-清除算法

use std::collections::HashMap as 哈希映射;

#[derive(Debug, Clone)]
pub enum 堆数据 {
    字符串(String),
    数组(Vec<usize>),
    映射(哈希映射<String, usize>),
}

#[derive(Debug, Clone)]
pub struct 堆对象 {
    pub 数据: 堆数据,
    pub 已标记: bool,
    pub 引用计数: usize,
}

pub struct 垃圾回收器 {
    堆: Vec<Option<堆对象>>,
    自由列表: Vec<usize>,
    分配计数: usize,
    回收阈值: usize,
}

impl 垃圾回收器 {
    pub fn 新建() -> Self {
        垃圾回收器 { 堆: Vec::new(), 自由列表: Vec::new(), 分配计数: 0, 回收阈值: 100 }
    }

    pub fn 分配(&mut self, 数据: 堆数据) -> usize {
        self.分配计数 += 1;
        if let Some(索引) = self.自由列表.pop() {
            self.堆[索引] = Some(堆对象 { 数据, 已标记: false, 引用计数: 1 });
            return 索引;
        }
        let 索引 = self.堆.len();
        self.堆.push(Some(堆对象 { 数据, 已标记: false, 引用计数: 1 }));
        if self.分配计数 >= self.回收阈值 { self.分配计数 = 0; }
        索引
    }

    pub fn 增加引用(&mut self, 索引: usize) {
        if let Some(Some(对象)) = self.堆.get_mut(索引) { 对象.引用计数 += 1; }
    }

    pub fn 减少引用(&mut self, 索引: usize) {
        if let Some(Some(对象)) = self.堆.get_mut(索引) {
            对象.引用计数 -= 1;
            if 对象.引用计数 == 0 { self.堆[索引] = None; self.自由列表.push(索引); }
        }
    }

    pub fn 标记(&mut self, 根索引: &[usize]) {
        for 对象 in self.堆.iter_mut().flatten() { 对象.已标记 = false; }
        for &索引 in 根索引 { self.标记对象(索引); }
    }

    fn 标记对象(&mut self, 索引: usize) {
        if let Some(Some(对象)) = self.堆.get_mut(索引) {
            if 对象.已标记 { return; }
            对象.已标记 = true;
            match &对象.数据.clone() {
                堆数据::数组(元素) => { for &子 in 元素 { self.标记对象(子); } }
                堆数据::映射(m) => { for &子 in m.values() { self.标记对象(子); } }
                _ => {}
            }
        }
    }

    pub fn 清除(&mut self) -> usize {
        let mut 数量 = 0;
        for (索引, 对象) in self.堆.iter_mut().enumerate() {
            if let Some(堆对象) = 对象 {
                if !堆对象.已标记 { *对象 = None; self.自由列表.push(索引); 数量 += 1; }
            }
        }
        数量
    }

    pub fn 执行回收(&mut self, 根索引: &[usize]) -> usize {
        self.标记(根索引);
        let 数量 = self.清除();
        self.分配计数 = 0;
        数量
    }
}
