# 1. 确保 ch04_variable_bindings 目录存在并创建 bin 目录

mkdir -p ch04_variable_bindings/src/bin

# 2. 批量创建按 URL 和小节规律命名的练习文件

# 4. 变量绑定概览 (variable_bindings.html)

touch ch04_variable_bindings/src/bin/0_variable_bindings.rs

# 4.1 可变性 (variable_bindings/mut.html)

touch ch04_variable_bindings/src/bin/1_mut.rs

# 4.2 作用域和遮蔽 (variable_bindings/scope.html)

touch ch04_variable_bindings/src/bin/2_scope.rs

# 4.3 先声明 (variable_bindings/declare.html)

touch ch04_variable_bindings/src/bin/3_declare.rs

# 4.4 冻结 (variable_bindings/freeze.html)

touch ch04_variable_bindings/src/bin/4_freeze.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch04_variable_bindings/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/variable_bindings.html

```
 *  正在执行任务: cargo run --package ch04_variable_bindings --bin 0_variable_bindings

   Compiling ch04_variable_bindings v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch04_variable_bindings)
warning: unused variable: `noisy_unused_variable`
  --> ch04_variable_bindings/src/bin/0_variable_bindings.rs:17:9
   |
17 |     let noisy_unused_variable = 2u32;
   |         ^^^^^^^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_noisy_unused_variable`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `ch04_variable_bindings` (bin "0_variable_bindings") generated 1 warning (run `cargo fix --bin "0_variable_bindings" -p ch04_variable_bindings` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/0_variable_bindings`
整数：1
布尔值：true
```

#### https://doc.rust-lang.org/rust-by-example/zh/variable_bindings/mut.html

```
   Compiling ch04_variable_bindings v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch04_variable_bindings)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/1_mut`
修改前：1
修改后：2
```

#### https://doc.rust-lang.org/rust-by-example/zh/variable_bindings/scope.html

```
   Compiling ch04_variable_bindings v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch04_variable_bindings)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/2_scope`
内部 short：2
外部 long：1
被遮蔽前：1
内部代码块中被遮蔽：abc
内部代码块外：1
外部代码块中被遮蔽：2
```

#### https://doc.rust-lang.org/rust-by-example/zh/variable_bindings/declare.html

```
   Compiling ch04_variable_bindings v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch04_variable_bindings)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/3_declare`
绑定：4
另一个绑定：1
```

#### https://doc.rust-lang.org/rust-by-example/zh/variable_bindings/freeze.html

```
   Compiling ch04_variable_bindings v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch04_variable_bindings)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/4_freeze`
 *  终端将被任务重用，按任意键关闭。
```
