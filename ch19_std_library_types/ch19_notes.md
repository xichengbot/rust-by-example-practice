# 1. 确保 ch19_std_library_types 目录存在并创建 bin 目录

mkdir -p ch19_std_library_types/src/bin

# 2. 批量创建按 URL 规律命名的练习文件

# 19.1 Box, stack and heap (std/box.html)

touch ch19_std_library_types/src/bin/1_box.rs

# 19.2 Vectors (std/vec.html)

touch ch19_std_library_types/src/bin/2_vec.rs

# 19.3 Strings (std/str.html)

touch ch19_std_library_types/src/bin/3_str.rs

# 19.4 Option (std/option.html)

touch ch19_std_library_types/src/bin/4_option.rs

# 19.5 Result 及其子节 (std/result.html, std/result/question_mark.html)

touch ch19_std_library_types/src/bin/5_result.rs
touch ch19_std_library_types/src/bin/5_result_question_mark.rs

# 19.6 panic! (std/panic.html)

touch ch19_std_library_types/src/bin/6_panic.rs

# 19.7 HashMap 及其子节 (std/hash.html, std/hash/alt_key_types.html, std/hash/hashset.html)

touch ch19_std_library_types/src/bin/7_hash.rs
touch ch19_std_library_types/src/bin/7_hash_alt_key_types.rs
touch ch19_std_library_types/src/bin/7_hash_hashset.rs

# 19.8 Rc (std/rc.html)

touch ch19_std_library_types/src/bin/8_rc.rs

# 19.9 Arc (std/arc.html)

touch ch19_std_library_types/src/bin/9_arc.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch19_std_library_types/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/std/box.html

```
 *  正在执行任务: cargo run --package ch19_std_library_types --bin 1_box

   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/1_box`
Point 在栈上占用 16 字节
Rectangle 在栈上占用 32 字节
装箱的 point 在栈上占用 8 字节
装箱的 rectangle 在栈上占用 8 字节
装箱的 box 在栈上占用 8 字节
未装箱的 point 在栈上占用 16 字节
```

#### https://doc.rust-lang.org/rust-by-example/zh/std/rc.html

```
正在执行任务: cargo run --package ch19_std_library_types --bin 8_rc

   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/8_rc`
--- rc_a 已创建 ---
rc_a 的引用计数：1
--- rc_a 被克隆为 rc_b ---
rc_b 的引用计数：2
rc_a 的引用计数：2
rc_a 和 rc_b 是否相等：true
rc_a 内部值的长度：9
rc_b 的值：Rc 示例
--- rc_b 超出作用域被释放 ---
rc_a 的引用计数：1
--- rc_a 超出作用域被释放 ---

```

#### https://doc.rust-lang.org/rust-by-example/zh/std/arc.html

```
 *  正在执行任务: cargo run --package ch19_std_library_types --bin 9_arc

   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s
     Running `target/debug/9_arc`
"同一个苹果"
"同一个苹果"
"同一个苹果"
"同一个苹果"
"同一个苹果"
"同一个苹果"
"同一个苹果"
"同一个苹果"
"同一个苹果"
"同一个苹果"
```

#### https://doc.rust-lang.org/rust-by-example/zh/std/vec.html

```
   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/2_vec`
将 (0..10) 收集到：[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
初始向量：[1, 2, 3]
将 4 推入向量
向量：[1, 2, 3, 4]
向量长度：4
第二个元素：2
弹出最后一个元素：Some(4)
xs 的内容：
> 1
> 2
> 3
在位置 0 的值是 1
在位置 1 的值是 2
在位置 2 的值是 3
更新后的向量：[3, 6, 9]
```

#### https://doc.rust-lang.org/rust-by-example/zh/std/str.html

```
   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/3_str`
全字母句：敏捷的棕色狐狸跳过懒惰的狗狗
单词逆序
> 敏捷的棕色狐狸跳过懒惰的狗狗
使用的字符：惰, 懒, 捷, 敏, 棕, 狐, 狗, 狸, 的, 色, 跳, 过
爱丽丝说：我喜欢狗
鲍勃说：我喜欢猫

```

#### https://doc.rust-lang.org/rust-by-example/zh/std/option.html

```
   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
warning: unused variable: `none`
  --> ch19_std_library_types/src/bin/4_option.rs:28:9
   |
28 |     let none: Option<i32> = None;
   |         ^^^^
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default
help: you might have meant to pattern match on the similarly named variant `None`
   |
28 -     let none: Option<i32> = None;
28 +     let std::prelude::v1::None: Option<i32> = None;
   |
help: if this is intentional, prefix it with an underscore
   |
28 |     let _none: Option<i32> = None;
   |         +

warning: `ch19_std_library_types` (bin "4_option") generated 1 warning (run `cargo fix --bin "4_option" -p ch19_std_library_types` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/4_option`
4 / 2 = 2
1 / 0 失败！
Some(0.0) 解包后得到 0.0
```

#### https://doc.rust-lang.org/rust-by-example/zh/std/result.html

```
 *  正在执行任务: cargo run --package ch19_std_library_types --bin 5_result

   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/5_result`

thread 'main' (7527149) panicked at ch19_std_library_types/src/bin/5_result.rs:47:29:
NegativeSquareRoot
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:80:14
   2: _5_result::op
             at ./ch19_std_library_types/src/bin/5_result.rs:47:29
   3: _5_result::main
             at ./ch19_std_library_types/src/bin/5_result.rs:56:20
   4: core::ops::function::FnOnce::call_once
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

#### https://doc.rust-lang.org/rust-by-example/zh/std/result/question_mark.html

```
 *  正在执行任务: cargo run --package ch19_std_library_types --bin 5_result_question_mark

   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/5_result_question_mark`

thread 'main' (7554267) panicked at ch19_std_library_types/src/bin/5_result_question_mark.rs:48:25:
负数的平方根
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:80:14
   2: core::panicking::panic_display
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/panicking.rs:259:5
   3: _5_result_question_mark::checked::op
             at ./ch19_std_library_types/src/bin/5_result_question_mark.rs:48:25
   4: _5_result_question_mark::main
             at ./ch19_std_library_types/src/bin/5_result_question_mark.rs:62:5
   5: core::ops::function::FnOnce::call_once
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

#### https://doc.rust-lang.org/rust-by-example/zh/std/panic.html

```
 *  正在执行任务: cargo run --package ch19_std_library_types --bin 6_panic

   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/6_panic`

thread 'main' (7558620) panicked at ch19_std_library_types/src/bin/6_panic.rs:5:9:
除以零
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:80:14
   2: _6_panic::division
             at ./ch19_std_library_types/src/bin/6_panic.rs:5:9
   3: _6_panic::main
             at ./ch19_std_library_types/src/bin/6_panic.rs:17:5
   4: core::ops::function::FnOnce::call_once
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

#### https://doc.rust-lang.org/rust-by-example/zh/std/hash.html

```
   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/7_hash`
正在呼叫 Daniel：很抱歉，无法接通您拨打的电话。
            请挂机后重试。
正在呼叫 Ashley：您好，这里是 Awesome 先生的披萨店。我是 Fred。
            请问今天您想点些什么？
正在呼叫Robert：嗨！请问是哪位？
正在呼叫Daniel：嗨！请问是哪位？
正在呼叫Katie：嗨！请问是哪位？
```

#### https://doc.rust-lang.org/rust-by-example/zh/std/hash/alt_key_types.html

```
 *  正在执行任务: cargo run --package ch19_std_library_types --bin 7_hash_alt_key_types

   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s
     Running `target/debug/7_hash_alt_key_types`
用户名：j.everyman
密码：psasword123
正在尝试登录...
登录失败！
用户名：j.everyman
密码：password123
正在尝试登录...
登录成功！
姓名：John Everyman
电子邮箱：j.everyman@email.com
```

#### https://doc.rust-lang.org/rust-by-example/zh/std/hash/hashset.html

```
   Compiling ch19_std_library_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch19_std_library_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/7_hash_hashset`
A：{1, 3, 2, 4}
B：{2, 4, 3, 5}
并集：[1, 3, 2, 4, 5]
差集：[1]
交集：[3, 2, 4]
对称差：[1, 5]
```
