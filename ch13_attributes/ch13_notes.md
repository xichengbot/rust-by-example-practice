# 1. 确保 ch13_attributes 目录存在并创建 bin 目录

mkdir -p ch13_attributes/src/bin

# 2. 批量创建按 URL 和小节规律命名的练习文件

# 13. 属性概览 (attribute.html)

touch ch13_attributes/src/bin/0_attribute.rs

# 13.1 dead_code (attribute/unused.html)

touch ch13_attributes/src/bin/1_unused.rs

# 13.2 Crates (attribute/crate.html)

# 注意：这一节通常演示的是修改 lib.rs 的属性，但为了保持格式统一，可以放在 bin 下测试

touch ch13_attributes/src/bin/2_crate.rs

# 13.3 cfg 及其子节 (attribute/cfg.html, attribute/cfg/custom.html)

touch ch13_attributes/src/bin/3_cfg.rs
touch ch13_attributes/src/bin/3_cfg_custom.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch13_attributes/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/attribute/unused.html

```
 *  正在执行任务: cargo run --package ch13_attributes --bin 1_unused

    Blocking waiting for file lock on artifact directory
   Compiling ch13_attributes v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch13_attributes)
warning: function `noisy_unused_function` is never used
 --> ch13_attributes/src/bin/1_unused.rs:7:4
  |
7 | fn noisy_unused_function() {}
  |    ^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch13_attributes` (bin "1_unused") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/1_unused`
```

#### https://doc.rust-lang.org/rust-by-example/zh/attribute/crate.html

#### https://doc.rust-lang.org/rust-by-example/zh/attribute/cfg.html

```
 *  正在执行任务: cargo run --package ch13_attributes --bin 3_cfg

   Compiling ch13_attributes v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch13_attributes)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/3_cfg`
你**不是**在运行 Linux！
你确定吗？
是的，这绝对**不是** Linux！
```

#### https://doc.rust-lang.org/rust-by-example/zh/attribute/cfg/custom.html

```
$ rustc --cfg some_condition custom.rs && ./custom
condition met!

```
