# 1. 确保 ch16_traits 目录存在并创建 bin 目录

mkdir -p ch16_traits/src/bin

# 2. 批量创建按 URL 规律命名的练习文件

# 16.1 Derive (trait/derive.html)

touch ch16_traits/src/bin/1_derive.rs

# 16.2 Returning Traits with dyn (trait/dyn.html)

touch ch16_traits/src/bin/2_dyn.rs

# 16.3 Operator Overloading (trait/ops.html)

touch ch16_traits/src/bin/3_ops.rs

# 16.4 Drop (trait/drop.html)

touch ch16_traits/src/bin/4_drop.rs

# 16.5 Iterators (trait/iter.html)

touch ch16_traits/src/bin/5_iter.rs

# 16.6 impl Trait (trait/impl_trait.html)

touch ch16_traits/src/bin/6_impl_trait.rs

# 16.7 Clone (trait/clone.html)

touch ch16_traits/src/bin/7_clone.rs

# 16.8 Supertraits (trait/supertraits.html)

touch ch16_traits/src/bin/8_supertraits.rs

# 16.9 Disambiguating overlapping traits (trait/disambiguating.html)

touch ch16_traits/src/bin/9_disambiguating.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch16_traits/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/trait/drop.html

```
正在执行任务: cargo run --package ch16_traits --bin 4_drop

   Compiling ch16_traits v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch16_traits)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.45s
     Running `target/debug/4_drop`
正在退出块 B
> 正在释放 d
> 正在释放 c
刚刚退出了块 B
正在退出块 A
> 正在释放 b
刚刚退出了块 A
> 正在释放 a
main 函数结束
 *  终端将被任务重用，按任意键关闭。

```

#### https://doc.rust-lang.org/rust-by-example/zh/trait.html

```
 *  正在执行任务: cargo run --package ch16_traits --bin 0_trait

   Compiling ch16_traits v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch16_traits)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/0_trait`
多莉 短暂停顿…… 咩～！
多莉 剃了个毛！
多莉 短暂停顿…… 咩～？
```

#### https://doc.rust-lang.org/rust-by-example/zh/trait/derive.html

```
 *  正在执行任务: cargo run --package ch16_traits --bin 1_derive

    Blocking waiting for file lock on artifact directory
   Compiling ch16_traits v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch16_traits)
warning: field `0` is never read
  --> ch16_traits/src/bin/1_derive.rs:18:16
   |
18 | struct Seconds(i32);
   |        ------- ^^^
   |        |
   |        field in this struct
   |
   = help: consider removing this field
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch16_traits` (bin "1_derive") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/1_derive`
一英尺等于 Inches(12)
一英尺比一米更小
```

#### https://doc.rust-lang.org/rust-by-example/zh/trait/dyn.html

```
 *  正在执行任务: cargo run --package ch16_traits --bin 2_dyn

   Compiling ch16_traits v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch16_traits)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/2_dyn`
你随机选择了一个动物，它说：咩～！
```

#### https://doc.rust-lang.org/rust-by-example/zh/trait/ops.html

```
  正在执行任务: cargo run --package ch16_traits --bin 3_ops

   Compiling ch16_traits v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch16_traits)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/3_ops`
> 调用了 Foo.add(Bar)
Foo + Bar = FooBar
> 调用了 Bar.add(Foo)
Bar + Foo = BarFoo
```

#### https://doc.rust-lang.org/rust-by-example/zh/trait/drop.html

```
   Compiling ch16_traits v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch16_traits)
warning: unused variable: `temp`
  --> ch16_traits/src/bin/4_drop.rs:43:13
   |
43 |         let temp = TempFile::new("test.txt".into())?;
   |             ^^^^ help: if this is intentional, prefix it with an underscore: `_temp`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: field `file` is never read
  --> ch16_traits/src/bin/4_drop.rs:61:5
   |
60 | struct TempFile {
   |        -------- field in this struct
61 |     file: File,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch16_traits` (bin "4_drop") generated 2 warnings (run `cargo fix --bin "4_drop" -p ch16_traits` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/4_drop`
正在退出块 B
> 正在释放 d
> 正在释放 c
刚刚退出了块 B
正在退出块 A
> 正在释放 b
刚刚退出了块 A
> 正在释放 a
main 函数结束
已创建临时文件
> 已丢弃临时文件："test.txt"
作用域结束 - 文件应该被清理
> 已丢弃临时文件："another_test.txt"
手动丢弃的文件
```

#### https://doc.rust-lang.org/rust-by-example/zh/trait/iter.html

```
 *  正在执行任务: cargo run --package ch16_traits --bin 5_iter

   Compiling ch16_traits v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch16_traits)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/5_iter`
对 0..3 连续调用四次 `next`
> Some(0)
> Some(1)
> Some(2)
> None
使用 `for` 遍历 0..3
> 0
> 1
> 2
斐波那契序列的前四项是：
> 0
> 1
> 1
> 2
斐波那契序列的接下来四项是：
> 3
> 5
> 8
> 13
遍历以下数组 [1, 3, 3, 7]
> 1
> 3
> 3
> 7
```

#### https://doc.rust-lang.org/rust-by-example/zh/trait/impl_trait.html

```
 *  正在执行任务: cargo run --package ch16_traits --bin 6_impl_trait

   Compiling ch16_traits v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch16_traits)
warning: function `combine_vecs_explicit_return_type` is never used
 --> ch16_traits/src/bin/6_impl_trait.rs:6:4
  |
6 | fn combine_vecs_explicit_return_type(
  |    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch16_traits` (bin "6_impl_trait") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/6_impl_trait`
全部完成
```

#### https://doc.rust-lang.org/rust-by-example/zh/trait/clone.html

```
 *  正在执行任务: cargo run --package ch16_traits --bin 7_clone

   Compiling ch16_traits v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch16_traits)
warning: fields `0` and `1` are never read
 --> ch16_traits/src/bin/7_clone.rs:9:13
  |
9 | struct Pair(Box<i32>, Box<i32>);
  |        ---- ^^^^^^^^  ^^^^^^^^
  |        |
  |        fields in this struct
  |
  = help: consider removing these fields
  = note: `Pair` has derived impls for the traits `Debug` and `Clone`, but these are intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch16_traits` (bin "7_clone") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
     Running `target/debug/7_clone`
原始: Unit
复制: Unit
原始: Pair(1, 2)
已移动: Pair(1, 2)
克隆: Pair(1, 2)
```

#### https://doc.rust-lang.org/rust-by-example/zh/trait/supertraits.html

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/8_supertraits`
我的名字是 Alice，我就读于 MIT。我最喜欢的语言是 Rust。我的 Git 用户名是 alice_codes
```

#### https://doc.rust-lang.org/rust-by-example/zh/trait/disambiguating.html

```
   Compiling ch16_traits v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch16_traits)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/9_disambiguating`
username is :rustacean
```
