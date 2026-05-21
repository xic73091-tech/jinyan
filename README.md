# 诚言 / Chengyan

**诚言** — 一门完全使用中文的编程语言，配有自研编译器与字节码虚拟机。

**Chengyan** — A fully Chinese programming language with a custom compiler and bytecode virtual machine.

---

## 特性 / Features

| 特性 | Feature |
|------|---------|
| 中文关键字与语法 | Chinese keywords and syntax |
| 递归下降语法分析器，优先级爬升 | Recursive descent parser with precedence climbing |
| 基于栈的字节码虚拟机 | Stack-based bytecode VM |
| 变量、函数、类定义 | Variables, functions, classes |
| 控制流：如果/否则、当循环、遍历、跳出/继续 | Control flow: if/else, while, for-each, break/continue |
| 数据类型：整数、小数、文本、布尔、数组、映射 | Types: int, float, string, bool, array, map |
| 内置函数：打印、长度、文本化、绝对值等 | Built-ins: print, length, to-string, abs, etc. |
| 类与方法（构造函数、实例属性、方法调用） | Classes (constructors, properties, methods) |
| 匹配表达式（模式匹配） | Match expressions (pattern matching) |
| 尝试/捕获（异常处理，简化版） | Try/catch (simplified exception handling) |

---

## 快速开始 / Quick Start

### 安装 / Install

需要 Rust 工具链 / Requires Rust toolchain:

```bash
# 安装 Rust / Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 编译与运行 / Build & Run

```bash
# 编译 / Build
cargo build --release

# 运行示例 / Run example
cargo run -- 示例.jy
```

### 示例代码 / Example Code

```
// 斐波那契 / Fibonacci
定义 斐波那契(n) {
    如果 n <= 1 { 返回 n }
    返回 斐波那契(n - 1) + 斐波那契(n - 2)
}

让 结果 = 斐波那契(10)
打印("斐波那契(10) = " + 文本化(结果))

// 遍历数组 / Iterate array
让 水果 = ["苹果", "香蕉", "橙子"]
对 水果名 在 水果 中 {
    打印("水果: " + 水果名)
}

// 类定义 / Class definition
类 点 {
    构造(x, y) {
        本.x = x
        本.y = y
    }
    方法 距离(其他) {
        让 dx = 本.x - 其他.x
        让 dy = 本.y - 其他.y
        返回 dx * dx + dy * dy
    }
}

让 p1 = 点(3, 4)
让 p2 = 点(0, 0)
打印("距离: " + 文本化(p1.距离(p2)))
```

---

## 语法概览 / Syntax Overview

| 诚言 | English equivalent |
|------|-------------------|
| `让 x = 值` | `let x = value` |
| `定义 函数名(参数) { ... }` | `fn function_name(params) { ... }` |
| `如果 条件 { ... } 否则 { ... }` | `if cond { ... } else { ... }` |
| `当 条件 { ... }` | `while cond { ... }` |
| `对 变量 在 集合 中 { ... }` | `for var in collection { ... }` |
| `返回 值` | `return value` |
| `类 名称 { ... }` | `class Name { ... }` |
| `本` | `self` |
| `真` / `假` | `true` / `false` |
| `且` / `或` / `非` | `and` / `or` / `not` |
| `跳出` / `继续` | `break` / `continue` |
| `匹配 值 { 分支 ... }` | `match value { arms ... }` |

---

## 项目结构 / Project Structure

```
src/
  主.rs          — 入口 / Entry point
  词法分析器.rs   — 词法分析器 / Lexer
  记号.rs        — 记号定义 / Token definitions
  语法分析器.rs   — 语法分析器 / Parser
  抽象语法树.rs   — AST 节点 / AST nodes
  编译器.rs      — 字节码编译器 / Bytecode compiler
  字节码.rs      — 字节码指令集 / Bytecode instructions
  虚拟机.rs      — 栈式虚拟机 / Stack VM
  值类型.rs      — 运行时值类型 / Runtime values
  内置函数.rs    — 内置函数库 / Built-in functions
  垃圾回收.rs    — GC（未集成）/ Garbage collector (not integrated)
  错误.rs        — 错误类型 / Error types
  宏.rs          — 辅助宏 / Helper macros
```

---

## 测试文件 / Test Files

| 文件 | 描述 / Description |
|------|-------------------|
| `示例.jy` | 基础示例 / Basic examples |
| `斐波那契.jy` | 斐波那契数列 / Fibonacci sequence |
| `杨辉三角.jy` | 杨辉三角 / Pascal's triangle |
| `新功能测试.jy` | 功能测试集 / Feature test suite |
| `类测试.jy` | 类与方法测试 / Class & method tests |

---

## 待办事项 / TODO

- [ ] 完善匹配表达式解析 / Complete match expression parsing
- [ ] 完善异常处理 / Complete exception handling
- [ ] 集成垃圾回收器 / Integrate garbage collector
- [ ] 安全加固（栈/槽位/常量池边界检查）/ Security hardening
- [ ] 继承实现 / Implement inheritance
- [ ] 模块/导入系统 / Module/import system

---

## 许可证 / License

MIT
