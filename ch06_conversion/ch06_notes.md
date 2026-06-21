# 1. 确保 ch06_conversion 目录存在并创建 bin 目录

mkdir -p ch06_conversion/src/bin

# 2. 批量创建按 URL 和小节规律命名的练习文件

# 6.1 From 和 Into (conversion/from_into.html)

touch ch06_conversion/src/bin/1_from_into.rs

# 6.2 TryFrom 和 TryInto (conversion/try_from_try_into.html)

touch ch06_conversion/src/bin/2_try_from_try_into.rs

# 6.3 转化为 String (conversion/string.html)

touch ch06_conversion/src/bin/3_string.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch06_conversion/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/conversion/from_into.html

```
warning: field `value` is never read
 --> ch06_conversion/src/bin/1_from_into.rs:5:5
  |
4 | struct Number {
  |        ------ field in this struct
5 |     value: i32,
  |     ^^^^^
  |
  = note: `Number` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch06_conversion` (bin "1_from_into") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/1_from_into`
我的数字是 Number { value: 30 }
```

#### https://doc.rust-lang.org/rust-by-example/zh/conversion/from_into.html

```
   Compiling ch06_conversion v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch06_conversion)
warning: field `value` is never read
 --> ch06_conversion/src/bin/1_from_into.rs:5:5
  |
4 | struct Number {
  |        ------ field in this struct
5 |     value: i32,
  |     ^^^^^
  |
  = note: `Number` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch06_conversion` (bin "1_from_into") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/1_from_into`
我的数字是 Number { value: 30 }
转换为Number ：Number { value: 5 }
```

#### https://doc.rust-lang.org/rust-by-example/zh/conversion/try_from_try_into.html

```
 *  正在执行任务: cargo run --package ch06_conversion --bin 2_try_from_try_into

   Compiling ch06_conversion v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch06_conversion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/2_try_from_try_into`
```

#### https://doc.rust-lang.org/rust-by-example/zh/conversion/string.html

```
 *  正在执行任务: cargo run --package ch06_conversion --bin 3_string

    Blocking waiting for file lock on artifact directory
   Compiling ch06_conversion v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch06_conversion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/3_string`
半径为 6 的圆
```

#### https://doc.rust-lang.org/rust-by-example/zh/conversion/string.html

```
   Compiling ch06_conversion v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch06_conversion)
warning: field `radius` is never read
  --> ch06_conversion/src/bin/2_try_from_try_into.rs:25:5
   |
24 | struct Circle {
   |        ------ field in this struct
25 |     radius: i32,
   |     ^^^^^^
   |
   = note: `Circle` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch06_conversion` (bin "2_try_from_try_into") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/2_try_from_try_into`
总和：15
Circle { radius: 3 }
```
