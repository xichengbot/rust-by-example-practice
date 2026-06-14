# 1. 确保 ch22_unsafe_operations 目录存在并创建 bin 目录

mkdir -p ch22_unsafe_operations/src/bin

# 2. 创建按 URL 和小节规律命名的练习文件

# 22. Unsafe 概览页 (unsafe.html)

# 这一页包含了 Raw Pointers (裸指针) 和 Calling Unsafe Functions 的代码示例

touch ch22_unsafe_operations/src/bin/0_unsafe.rs

# 22.1 Inline assembly (unsafe/asm.html)

touch ch22_unsafe_operations/src/bin/1_asm.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch22_unsafe_operations/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/unsafe.html

```
   Compiling ch22_unsafe_operations v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch22_unsafe_operations)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/0_unsafe`
 *
```

#### https://doc.rust-lang.org/rust-by-example/zh/unsafe/asm.html

```
   Compiling ch22_unsafe_operations v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch22_unsafe_operations)
warning: unnecessary `unsafe` block
  --> ch22_unsafe_operations/src/bin/1_asm.rs:76:5
   |
76 |     unsafe {
   |     ^^^^^^ unnecessary `unsafe` block
   |
   = note: `#[warn(unused_unsafe)]` (part of `#[warn(unused)]`) on by default

warning: function `call_foo` is never used
 --> ch22_unsafe_operations/src/bin/1_asm.rs:9:4
  |
9 | fn call_foo(arg: i32) -> i32 {
  |    ^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch22_unsafe_operations` (bin "1_asm") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/1_asm`
CPU 制造商 ID：GenuineIntel
arg = -3
 call foo of C: -6
```
