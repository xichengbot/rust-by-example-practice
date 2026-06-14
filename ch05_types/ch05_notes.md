# 1. 确保 ch05 目录存在并创建 bin 目录

mkdir -p ch05_types/src/bin

# 2. 批量创建按 URL 规律命名的练习文件

# 5.1 Casting (types/cast.html)

touch ch05_types/src/bin/1_cast.rs

# 5.2 Literals (types/literals.html)

touch ch05_types/src/bin/2_literals.rs

# 5.3 Inference (types/inference.html)

touch ch05_types/src/bin/3_inference.rs

# 5.4 Aliasing (types/alias.html)

touch ch05_types/src/bin/4_alias.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch05_types/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/types/cast.html

```
正在执行任务: cargo run --package ch05_types --bin 1_cast

   Compiling ch05_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch05_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/1_cast`
类型转换：65.4321 -> 65 -> A
1000 转换为 u16 是：1000
1000 转换为 u8 是：232
  -1 转换为 u8 是：255
1000 对 256 取模是：232
 128 转换为 i16 是：128
 128 转换为 i8 是：-128
1000 转换为 u8 是：232
 232 转换为 i8 是：-24
 300.0 转换为 u8 是：255
-100.0 转换为 u8 是：0
   NaN 转换为 u8 是：0
 300.0 转换为 u8 是：44
-100.0 转换为 u8 是：156
   NaN 转换为 u8 是：0
```

#### https://doc.rust-lang.org/rust-by-example/zh/types/literals.html

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/2_literals`
`x` 的字节大小：1
`y` 的字节大小：4
`z` 的字节大小：4
`i` 的字节大小：4
`f` 的字节大小：8
```

#### https://doc.rust-lang.org/rust-by-example/zh/types/inference.html

```
 *  正在执行任务: cargo run --package ch05_types --bin 3_inference

   Compiling ch05_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch05_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/3_inference`
[5]
```

#### https://doc.rust-lang.org/rust-by-example/zh/types/alias.html

```
正在执行任务: cargo run --package ch05_types --bin 4_alias

   Compiling ch05_types v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch05_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/4_alias`
5 纳秒 + 2 英寸 = 7 单位？
```
