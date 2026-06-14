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
