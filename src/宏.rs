/// 中文宏 —— 封装标准库输出函数

macro_rules! 打印行 {
    ($($参数:tt)*) => { println!($($参数)*) };
}

macro_rules! 打印 {
    ($($参数:tt)*) => { print!($($参数)*) };
}

macro_rules! 错误输出 {
    ($($参数:tt)*) => { eprintln!($($参数)*) };
}

macro_rules! 格式化 {
    ($($参数:tt)*) => { format!($($参数)*) };
}
