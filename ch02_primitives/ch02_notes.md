chapters=(
"ch02_primitives"
"ch03_custom_types"
"ch04_variable_bindings"
"ch05_types"
"ch06_conversion"
"ch07_expressions"
"ch10_modules"
"ch11_crates"
"ch12_cargo"
"ch13_attributes"
"ch16_traits"
"ch17_macros"
"ch18_error_handling"
"ch19_std_library_types"
"ch20_std_misc"
"ch21_testing"
"ch22_unsafe_operations"
"ch23_compatibility"
"ch24_meta"
)

for ch in "${chapters[@]}"; do
  if [ ! -d "$ch" ]; then
cargo new "$ch"
else
echo "✅ 目录 $ch 已存在，跳过..."
fi
done

# 1. 确保 ch02_primitives 目录存在并创建 bin 目录

mkdir -p ch02_primitives/src/bin

# 2. 批量创建按 URL 和小节规律命名的练习文件

# 2. 原生类型概览 (primitives.html)

touch ch02_primitives/src/bin/0_primitives.rs

# 2.1 字面量和运算符 (primitives/literals.html)

touch ch02_primitives/src/bin/1_literals.rs

# 2.2 元组 (primitives/tuples.html)

touch ch02_primitives/src/bin/2_tuples.rs

# 2.3 数组和切片 (primitives/array.html)

touch ch02_primitives/src/bin/3_array.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch02_primitives/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/primitives.html

```
 Blocking waiting for file lock on artifact directory
   Compiling ch02_primitives v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch02_primitives)
warning: unused variable: `logical`
 --> ch02_primitives/src/bin/0_primitives.rs:3:9
  |
3 |     let logical: bool = true;
  |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_logical`
  |
  = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: unused variable: `a_float`
 --> ch02_primitives/src/bin/0_primitives.rs:5:9
  |
5 |     let a_float: f64 = 1.0;  // 常规标注
  |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_a_float`

warning: unused variable: `an_integer`
 --> ch02_primitives/src/bin/0_primitives.rs:6:9
  |
6 |     let an_integer   = 5i32; // 后缀标注
  |         ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_an_integer`

warning: unused variable: `default_float`
 --> ch02_primitives/src/bin/0_primitives.rs:9:9
  |
9 |     let default_float   = 3.0; // `f64`
  |         ^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_default_float`

warning: unused variable: `default_integer`
  --> ch02_primitives/src/bin/0_primitives.rs:10:9
   |
10 |     let default_integer = 7;   // `i32`
   |         ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_default_integer`

warning: variable `inferred_type` is assigned to, but never used
  --> ch02_primitives/src/bin/0_primitives.rs:13:9
   |
13 |     let mut inferred_type = 12; // 从另一行推断出类型为 i64。
   |         ^^^^^^^^^^^^^^^^^
   |
   = note: consider using `_inferred_type` instead

warning: variable `mutable` is assigned to, but never used
  --> ch02_primitives/src/bin/0_primitives.rs:17:9
   |
17 |     let mut mutable = 12; // 可变的 `i32`
   |         ^^^^^^^^^^^
   |
   = note: consider using `_mutable` instead

warning: unused variable: `mutable`
  --> ch02_primitives/src/bin/0_primitives.rs:24:9
   |
24 |     let mutable = true;
   |         ^^^^^^^ help: if this is intentional, prefix it with an underscore: `_mutable`

warning: unused variable: `my_array`
  --> ch02_primitives/src/bin/0_primitives.rs:29:9
   |
29 |     let my_array: [i32; 5] = [1, 2, 3, 4, 5];
   |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_my_array`

warning: unused variable: `my_tuple`
  --> ch02_primitives/src/bin/0_primitives.rs:33:9
   |
33 |     let my_tuple = (5u32, 1u8, true, -5.04f32);
   |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_my_tuple`

warning: value assigned to `inferred_type` is never read
  --> ch02_primitives/src/bin/0_primitives.rs:14:5
   |
14 |     inferred_type = 4294967296i64;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: maybe it is overwritten before being read?
   = note: `#[warn(unused_assignments)]` (part of `#[warn(unused)]`) on by default

warning: value assigned to `mutable` is never read
  --> ch02_primitives/src/bin/0_primitives.rs:18:5
   |
18 |     mutable = 21;
   |     ^^^^^^^^^^^^
   |
   = help: maybe it is overwritten before being read?

warning: `ch02_primitives` (bin "0_primitives") generated 12 warnings (run `cargo fix --bin "0_primitives" -p ch02_primitives` to apply 8 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.03s
     Running `target/debug/0_primitives`
```

#### https://doc.rust-lang.org/rust-by-example/zh/primitives/literals.html

```
 *  正在执行任务: cargo run --package ch02_primitives --bin 1_literals

   Compiling ch02_primitives v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch02_primitives)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/1_literals`
1 + 2 = 3
1 - 2 = -1
1e4 is 10000, -2.5e-3 is -0.0025
true AND false is false
true OR false is true
NOT true is false
0011 AND 0101 is 0001
0011 OR 0101 is 0111
0011 XOR 0101 is 0110
1 << 5 is 32
0x80 >> 2 is 0x20
One million is written as 1000000
```

#### https://doc.rust-lang.org/rust-by-example/zh/primitives/tuples.html

```
  正在执行任务: cargo run --package ch02_primitives --bin 2_tuples

   Compiling ch02_primitives v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch02_primitives)
warning: fields `0`, `1`, `2`, and `3` are never read
  --> ch02_primitives/src/bin/2_tuples.rs:11:15
   |
11 | struct Matrix(f32, f32, f32, f32);
   |        ------ ^^^  ^^^  ^^^  ^^^
   |        |
   |        fields in this struct
   |
   = help: consider removing these fields
   = note: `Matrix` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch02_primitives` (bin "2_tuples") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/2_tuples`
长元组的第一个值：1
长元组的第二个值：2
元组的元组：((1, 2, 2), (4, -1), -2)
对组是 (1, true)
反转后的对组是 (true, 1)
单元素元组：(5,)
仅是一个整数：5
1、"hello"、4.5、true
Matrix(1.1, 1.2, 2.1, 2.2)
```

#### https://doc.rust-lang.org/rust-by-example/zh/primitives/array.html

```
 *  正在执行任务: cargo run --package ch02_primitives --bin 3_array

   Compiling ch02_primitives v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch02_primitives)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/3_array`
数组的第一个元素：1
数组的第二个元素：2
数组中的元素数量：5
数组占用 20 字节
将整个数组借用为切片。
切片的第一个元素：1
切片有 5 个元素
借用数组的一部分作为切片。
切片的第一个元素：0
切片有 3 个元素
0: 1
1: 2
2: 3
3: 4
4: 5
慢着！5 超出范围了！
```
