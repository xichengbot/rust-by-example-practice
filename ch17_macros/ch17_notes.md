# 1. 确保 ch17_macros 目录存在并创建 bin 目录

mkdir -p ch17_macros/src/bin

# 2. 批量创建按 URL 规律命名的练习文件

# 17. 宏概览 (macros.html)

touch ch17_macros/src/bin/0_macros.rs

# 17.1 Syntax 及其子节 (macros/syntax.html, macros/designators.html, macros/overload.html, macros/repeat.html)

touch ch17_macros/src/bin/1_syntax.rs
touch ch17_macros/src/bin/1_syntax_designators.rs
touch ch17_macros/src/bin/1_syntax_overload.rs
touch ch17_macros/src/bin/1_syntax_repeat.rs

# 17.2 DRY (macros/dry.html)

touch ch17_macros/src/bin/2_dry.rs

# 17.3 DSL (macros/dsl.html)

touch ch17_macros/src/bin/3_dsl.rs

# 17.4 Variadics (macros/variadics.html)

touch ch17_macros/src/bin/4_variadics.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch17_macros/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/macros.html

```
 *  正在执行任务: cargo run --package ch17_macros --bin 0_macros

   Compiling ch17_macros v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch17_macros)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/0_macros`
Hello!
```

#### https://doc.rust-lang.org/rust-by-example/zh/macros/designators.html

```
 *  正在执行任务: cargo run --package ch17_macros --bin 1_syntax_designators

   Compiling ch17_macros v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch17_macros)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/1_syntax_designators`
你调用了 "foo"()
你调用了 "bar"()
"1u32 + 1" = 2
"{ let x = 1u32; x * x + 2 * x - 1 }" = 2
```

#### https://doc.rust-lang.org/rust-by-example/zh/macros/overload.html

```
 *  正在执行任务: cargo run --package ch17_macros --bin 1_syntax_overload

   Compiling ch17_macros v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch17_macros)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/1_syntax_overload`
"1i32 + 1 == 2i32" 和 "2i32 * 2 == 4i32" 是 true
"true" 或 "false" 是 true
```

#### https://doc.rust-lang.org/rust-by-example/zh/macros/repeat.html

```
 *  正在执行任务: cargo run --package ch17_macros --bin 1_syntax_repeat

   Compiling ch17_macros v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch17_macros)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/1_syntax_repeat`
1
2
4
```

#### https://doc.rust-lang.org/rust-by-example/zh/macros/dry.html

####

```
 *  正在执行任务: cargo run --package ch17_macros --bin 3_dsl

   Compiling ch17_macros v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch17_macros)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/3_dsl`
1 + 2 = 3
(1 + 2) * (3 / 4) = 0
```

#### https://doc.rust-lang.org/rust-by-example/zh/macros/variadics.html

```

   Compiling ch17_macros v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch17_macros)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/4_variadics`
1 + 2 = 3
3 + 4 = 7
```
