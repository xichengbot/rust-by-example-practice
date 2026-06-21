# 1. 确保 ch10_modules 目录存在并创建 bin 目录

mkdir -p ch10_modules/src/bin

# 2. 批量创建按 URL 和小节规律命名的练习文件

# 10.1 可见性 (mod/visibility.html)

touch ch10_modules/src/bin/1_visibility.rs

# 10.2 结构体的可见性 (mod/struct_visibility.html)

touch ch10_modules/src/bin/2_struct_visibility.rs

# 10.3 use 声明 (mod/use.html)

touch ch10_modules/src/bin/3_use.rs

# 10.4 super 和 self (mod/super.html)

touch ch10_modules/src/bin/4_super.rs

# 10.5 文件分层 (mod/split.html)

# 注意：这一节重点是演示模块在不同文件/目录间的拆分，

# 创建这个 bin 文件可以作为你的主入口，你可能还需要在 bin 目录下额外创建同名的目录来配合练习。

touch ch10_modules/src/bin/5_split.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch10_modules/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/mod/visibility.html

```
   Compiling ch10_modules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch10_modules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/1_visibility`
调用了 `function()`
调用了 `my_mod::function()`
调用了 `my_mod::indirect_access()`，它
> 调用了 `my_mod::private_function()`
调用了 `my_mod::nested::function()`
调用了 `my_mod::call_public_function_in_my_mod()`，它
> 调用了 `my_mod::nested::public_function_in_my_mod()`，它
> 调用了 `my_mod::nested::public_function_in_nested()`
> 调用了 `my_mod::nested::public_function_in_super_mod()`
调用了 `my_mod::public_function_in_crate()`
```

#### https://doc.rust-lang.org/rust-by-example/zh/mod/struct_visibility.html

```
 *  正在执行任务: cargo run --package ch10_modules --bin 2_struct_visibility

   Compiling ch10_modules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch10_modules)
warning: field `contents` is never read
 --> ch10_modules/src/bin/2_struct_visibility.rs:9:9
  |
8 |     pub struct ClosedBox<T> {
  |                --------- field in this struct
9 |         contents: T,
  |         ^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch10_modules` (bin "2_struct_visibility") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/2_struct_visibility`
打开的盒子包含：公开信息

```

#### https://doc.rust-lang.org/rust-by-example/zh/mod/use.html

```
 *  正在执行任务: cargo run --package ch10_modules --bin 3_use

   Compiling ch10_modules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch10_modules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/3_use`
调用了 `deeply::nested::function()`
进入代码块
调用了 `deeply::nested::function()`
离开代码块
调用了 `function()`
```

#### https://doc.rust-lang.org/rust-by-example/zh/mod/super.html

```
 *  正在执行任务: cargo run --package ch10_modules --bin 4_super

   Compiling ch10_modules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch10_modules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/4_super`
调用了 `my::indirect_call()`，它
> 调用了 `my::function()`
调用了 `my::function()`
调用了 `my::cool::function()`
调用了 `function()`
调用了 `cool::function()`
```
