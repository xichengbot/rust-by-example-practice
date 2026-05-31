cargo new ch15_scoping_rules

mkdir -p ch15_scoping_rules/src/bin

touch ch15_scoping_rules/src/bin/1_raii.rs
touch ch15_scoping_rules/src/bin/2_ownership.rs
touch ch15_scoping_rules/src/bin/3_borrowing.rs
touch ch15_scoping_rules/src/bin/4_lifetimes.rs

#### https://doc.rust-lang.org/rust-by-example/zh/scope/lifetime/explicit.html

```
正在执行任务: cargo run --package ch15_scoping_rules --bin 4_lifetimes

   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
error[E0597]: `_x` does not live long enough
  --> ch15_scoping_rules/src/bin/4_lifetimes.rs:13:23
   |
 9 | fn failed_borrow<'a>() {
   |                  -- lifetime `'a` defined here
10 |     let _x = 12;
   |         -- binding `_x` declared here
...
13 |     let _y: &'a i32 = &_x;
   |             -------   ^^^ borrowed value does not live long enough
   |             |
   |             type annotation requires that `_x` is borrowed for `'a`
...
17 | }
   | - `_x` dropped here while still borrowed

For more information about this error, try `rustc --explain E0597`.
error: could not compile `ch15_scoping_rules` (bin "4_lifetimes") due to 1 previous error

 *  终端进程“cargo 'run', '--package', 'ch15_scoping_rules', '--bin', '4_lifetimes'”启动失败(退出代码: 101)。

```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/lifetime/fn.html

```
 *  正在执行任务: cargo run --package ch15_scoping_rules --bin 4_lifetimes_functions

   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/4_lifetimes_functions`
`print_one`：x 是 7
`print_multi`：x 是 7，y 是 9
`print_one`：x 是 7
`print_one`：x 是 4
 *  终端将被任务重用，按任意键关闭。
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/lifetime/methods.html

```
*  正在执行任务: cargo run --package ch15_scoping_rules --bin 4_lifetimes_methods

   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/4_lifetimes_methods`
`print`：19
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/lifetime/struct.html

```

warning: `ch15_scoping_rules` (bin "4_lifetimes_structs") generated 4 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/4_lifetimes_structs`
x 在 Borrowed(18) 中被借用
x 和 y 在 NamedBorrowed { x: 18, y: 15 } 中被借用
x 在 Ref(18) 中被借用
y 在 Num(15) 中**没有**被借用

```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/lifetime/trait.html

```
warning: `ch15_scoping_rules` (bin "4_lifetimes_traits") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/4_lifetimes_traits`
b 是 Borrowed { x: 10 }
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/lifetime/lifetime_bounds.html

```
warning: `ch15_scoping_rules` (bin "4_lifetimes_bounds") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/4_lifetimes_bounds`
`print_ref`: t 是 Ref(7)
`print`: t 是 Ref(7)
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/lifetime/lifetime_coercion.html

```
正在执行任务: cargo run --package ch15_scoping_rules --bin 4_lifetimes_coercion

   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/4_lifetimes_coercion`
乘积是 6
2 是第一个
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/lifetime/static_lifetime.html

```
   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/4_lifetimes_static`
static_string 的值：我存储在只读内存中
coerced_static: 18
NUM：18 仍然可以访问！
传入的 'static 值是：5
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/lifetime/elision.html

```
正在执行任务: cargo run --package ch15_scoping_rules --bin 4_lifetimes_elision

   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/4_lifetimes_elision`
`elided_input`: 3
`annotated_input`: 3
`elided_pass`: 3
`annotated_pass`: 3
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/raii.html

```
   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
warning: unused variable: `x`
  --> ch15_scoping_rules/src/bin/1_raii.rs:35:9
   |
35 |     let x = ToDrop;
   |         ^ help: if this is intentional, prefix it with an underscore: `_x`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `ch15_scoping_rules` (bin "1_raii") generated 1 warning (run `cargo fix --bin "1_raii" -p ch15_scoping_rules` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/1_raii`
创建了一个 ToDrop！
ToDrop 正在被丢弃
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/move.html

```
    Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/2_ownership`
x 是 5，y 是 5
a 包含：5
正在销毁一个包含 5 的 box
immutable_box 包含 5
mutable_box 包含 5
mutable_box 现在包含 4
此人的年龄是 20
此人的姓名是 Alice
从 person 结构体中获取的年龄是 20
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/borrow.html

```
   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/3_borrowing`
这个整数是：5
这个整数是：5
这个整数是：6
i32
这个整数是：5
正在销毁包含 5 的盒子
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/borrow/mut.html

```
   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/3_borrow_mut`
我不可变地借用了《哥德尔、埃舍尔、巴赫》- 1979 版
我不可变地借用了《哥德尔、埃舍尔、巴赫》- 1979 版
我可变地借用了《哥德尔、埃舍尔、巴赫》- 2014 版
```

#### https://doc.rust-lang.org/rust-by-example/zh/scope/borrow/alias.html

```
    Blocking waiting for file lock on artifact directory
   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/3_borrow_alias`
点的坐标为：(0, 0, 0)
点的坐标为：(0, 0, 0)
点的坐标为：(5, 2, 1)
点现在的坐标为：(5, 2, 1)
```

### https://doc.rust-lang.org/rust-by-example/zh/scope/borrow/ref.html

```
 正在执行任务: cargo run --package ch15_scoping_rules --bin 3_borrow_ref

   Compiling ch15_scoping_rules v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch15_scoping_rules)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running `target/debug/3_borrow_ref`
ref_c1 等于 ref_c2：true
point 的坐标是 (0, 0)
mutable_point 的坐标是 (0, 1
```
