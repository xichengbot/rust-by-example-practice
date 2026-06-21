# 1. 确保 ch14_generics 目录存在并创建 bin 目录

mkdir -p ch14_generics/src/bin

# 2. 批量创建按 URL 和小节规律命名的练习文件

# 14. 泛型概览 (generics.html)

touch ch14_generics/src/bin/0_generics.rs

# 14.1 函数 (generics/gen_fn.html)

touch ch14_generics/src/bin/1_gen_fn.rs

# 14.2 实现 (generics/impl.html)

touch ch14_generics/src/bin/2_impl.rs

# 14.3 特质 (generics/gen_trait.html)

touch ch14_generics/src/bin/3_gen_trait.rs

# 14.4 约束 及其子节 (generics/bounds.html, generics/bounds/testcase_empty.html)

touch ch14_generics/src/bin/4_bounds.rs
touch ch14_generics/src/bin/4_bounds_testcase_empty.rs

# 14.5 多重约束 (generics/multi_bounds.html)

touch ch14_generics/src/bin/5_multi_bounds.rs

# 14.6 Where 分句 (generics/where.html)

touch ch14_generics/src/bin/6_where.rs

# 14.7 新类型惯用法 (generics/new_types.html)

touch ch14_generics/src/bin/7_new_types.rs

# 14.8 关联项 及其子节 (generics/assoc_items.html, ...)

touch ch14_generics/src/bin/8_assoc_items.rs
touch ch14_generics/src/bin/8_assoc_items_the_problem.rs
touch ch14_generics/src/bin/8_assoc_items_types.rs

# 14.9 虚类型参数 及其子节 (generics/phantom.html, ...)

touch ch14_generics/src/bin/9_phantom.rs
touch ch14_generics/src/bin/9_phantom_testcase_units.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch14_generics/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/generics.html

```
 *  正在执行任务: cargo run --package ch14_generics --bin 0_generics

   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/0_generics`
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/gen_fn.html

```
 *  正在执行任务: cargo run --package ch14_generics --bin 1_gen_fn

   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/1_gen_fn`
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/impl.html

```
 *  正在执行任务: cargo run --package ch14_generics --bin 2_impl

    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/2_impl`
3, 3
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/gen_trait.html

```
   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/3_gen_trait`
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/bounds.html

```
   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/4_bounds`
Rectangle { length: 3.0, height: 4.0 }
面积：12
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/bounds/testcase_empty.html

```
   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/4_bounds_testcase_empty`
红雀是红色
蓝松鸟是蓝色
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/multi_bounds.html

```
   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/5_multi_bounds`
Debug：`"words"`
Display：`words`
t：`[1, 2, 3]`
u：`[1, 2, 3]`
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/where.html

```
 *  正在执行任务: cargo run --package ch14_generics --bin 6_where

   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.02s
     Running `target/debug/6_where`
Some([1, 2, 3])
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/new_types.html

```
   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/7_new_types`
Is a marathon? true
Is a marathon? true
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/assoc_items/the_problem.html

```
 *  正在执行任务: cargo run --package ch14_generics --bin 8_assoc_items_the_problem

   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/8_assoc_items_the_problem`
容器是否包含 3 和 10：true
第一个数字：3
最后一个数字：10
差值为：7
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/assoc_items/types.html

```
 *  正在执行任务: cargo run --package ch14_generics --bin 8_assoc_items_types

   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/8_assoc_items_types`
容器是否包含 3 和 10：true
第一个数字：3
最后一个数字：10
差值为：7
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/phantom.html

```
   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/9_phantom`
_tuple1 == _tuple2 的结果是：true
```

#### https://doc.rust-lang.org/rust-by-example/zh/generics/phantom/testcase_units.html

```
   Compiling ch14_generics v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch14_generics)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/9_phantom_testcase_units`
一英尺 + 一英尺 = 24.0 英寸
一米 + 一米 = 2000.0 毫米
```
