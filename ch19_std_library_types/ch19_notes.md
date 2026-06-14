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
