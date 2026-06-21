# 1. 确保 ch07_expressions 目录存在并创建 bin 目录

mkdir -p ch07_expressions/src/bin

# 2. 为表达式概览创建单一的练习文件

touch ch07_expressions/src/bin/1_expression.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch07_expressions/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/expression.html

```
 *  正在执行任务: cargo run --package ch07_expressions --bin 1_expression

   Compiling ch07_expressions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch07_expressions)
warning: unused arithmetic operation that must be used
  --> ch07_expressions/src/bin/1_expression.rs:14:9
   |
14 |         2 * x;
   |         ^^^^^ the arithmetic operation produces a value
   |
   = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
   |
14 |         let _ = 2 * x;
   |         +++++++

warning: `ch07_expressions` (bin "1_expression") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/1_expression`
x 是 5
y 是 155
z 是 ()
```
