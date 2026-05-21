/// 抽象语法树 —— 诚言编程语言

/// 运算符类型
#[derive(Debug, Clone, PartialEq)]
pub enum 运算符类型 {
    加, 减, 乘, 除, 取余, 取反,
    等于, 不等于, 小于, 大于, 小于等于, 大于等于,
    逻辑与, 逻辑或, 逻辑非,
}

/// 表达式
#[derive(Debug, Clone)]
pub enum 表达式 {
    整数(i64),
    小数(f64),
    文本(String),
    布尔(bool),
    空值,
    标识符(String),
    二元运算 { 左: Box<表达式>, 运算符: 运算符类型, 右: Box<表达式> },
    一元运算 { 运算符: 运算符类型, 操作数: Box<表达式> },
    函数调用 { 函数: Box<表达式>, 参数: Vec<表达式> },
    成员访问 { 对象: Box<表达式>, 成员: String },
    索引访问 { 对象: Box<表达式>, 索引: Box<表达式> },
    数组字面量(Vec<表达式>),
    映射字面量(Vec<(表达式, 表达式)>),
    箭头函数 { 参数: Vec<参数定义>, 体: Box<语句> },
    本引用,
}

/// 语句
#[derive(Debug, Clone)]
pub enum 语句 {
    表达式语句(表达式),
    变量声明 { 名称: String, 初始值: Option<表达式>, 是否常量: bool, 类型标注: Option<String> },
    赋值 { 目标: 表达式, 值: 表达式 },
    如果 { 条件: 表达式, 真分支: Vec<语句>, 假分支: Option<Vec<语句>> },
    当循环 { 条件: 表达式, 体: Vec<语句> },
    遍历 { 变量名: String, 集合: 表达式, 体: Vec<语句> },
    返回(Option<表达式>),
    跳出,
    继续,
    函数定义 { 名称: String, 参数: Vec<参数定义>, 返回类型: Option<String>, 体: Vec<语句>, 是否构造: bool },
    类定义 { 名称: String, 父类: Option<String>, 接口列表: Vec<String>, 成员: Vec<类成员> },
    匹配 { 值: 表达式, 分支: Vec<匹配分支> },
    尝试捕获 { 体: Vec<语句>, 捕获变量: String, 捕获体: Vec<语句> },
    抛出(表达式),
    块(Vec<语句>),
}

/// 参数定义
#[derive(Debug, Clone)]
pub struct 参数定义 {
    pub 名称: String,
    pub 类型标注: Option<String>,
}

/// 类成员
#[derive(Debug, Clone)]
pub enum 类成员 {
    属性 { 名称: String, 类型标注: Option<String>, 是否公有: bool },
    方法 { 名称: String, 参数: Vec<参数定义>, 返回类型: Option<String>, 体: Vec<语句>, 是否公有: bool, 是否覆盖: bool },
    构造函数 { 参数: Vec<参数定义>, 体: Vec<语句> },
}

/// 匹配分支
#[derive(Debug, Clone)]
pub struct 匹配分支 {
    pub 模式: 匹配模式,
    pub 体: Vec<语句>,
}

/// 匹配模式
#[derive(Debug, Clone)]
pub enum 匹配模式 {
    字面量(表达式),
    标识符(String),
    通配符,
}

/// 程序根节点
#[derive(Debug)]
pub struct 程序 {
    pub 语句列表: Vec<语句>,
}

impl 程序 {
    pub fn 新建() -> Self {
        程序 { 语句列表: Vec::new() }
    }
}
