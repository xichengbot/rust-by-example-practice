cargo new ch09_functions

# 1. 确保 ch09 存在并且创建 bin 目录

mkdir -p ch09_functions/src/bin

# 2. 批量创建按 URL 规律命名的练习文件

# 9.1 Methods (fn/methods.html)

touch ch09_functions/src/bin/1_methods.rs

# 9.2 Closures 及其子目录 (fn/closures/\*.html)

touch ch09_functions/src/bin/2_closures.rs
touch ch09_functions/src/bin/2_closures_capture.rs
touch ch09_functions/src/bin/2_closures_input_parameters.rs
touch ch09_functions/src/bin/2_closures_anonymity.rs
touch ch09_functions/src/bin/2_closures_input_functions.rs
touch ch09_functions/src/bin/2_closures_output_parameters.rs

# 9.2.6 std 中的例子 (fn/closures/closure_examples/\*.html)

touch ch09_functions/src/bin/2_closures_iter_any.rs
touch ch09_functions/src/bin/2_closures_iter_find.rs

# 9.3 Higher Order Functions (fn/hof.html)

touch ch09_functions/src/bin/3_hof.rs

# 9.4 Diverging functions (fn/diverging.html)

touch ch09_functions/src/bin/4_diverging.rs

## closures

#### https://doc.rust-lang.org/rust-by-example/zh/fn/closures.html

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/2_closures`
closure_annotated：43
closure_inferred：43
返回 1 的闭包：1
```

#### https://doc.rust-lang.org/rust-by-example/zh/fn/closures/capture.html

```
   Compiling ch09_functions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch09_functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/2_closures_capture`
`color`: green
`color`: green
`count`: 1
`count`: 2
`movable`: 3
true
false
vec 中有 3 个元素
```

#### https://doc.rust-lang.org/rust-by-example/zh/fn/closures/input_parameters.html

```
 *  正在执行任务: cargo run --package ch09_functions --bin 2_closures_input_parameters

   Compiling ch09_functions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch09_functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/2_closures_input_parameters`
我说hello。
然后我喊goodbye！！！。
现在我可以睡觉了。呼呼
3 的两倍是：6
```

#### https://doc.rust-lang.org/rust-by-example/zh/fn/closures/anonymity.html

```
 *  正在执行任务: cargo run --package ch09_functions --bin 2_closures_anonymity

   Compiling ch09_functions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch09_functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/2_closures_anonymity`
7
```

#### https://doc.rust-lang.org/rust-by-example/zh/fn/closures/input_functions.html

```
 *  正在执行任务: cargo run --package ch09_functions --bin 2_closures_input_functions

   Compiling ch09_functions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch09_functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/2_closures_input_functions`
我是闭包！
我是函数！
```

#### https://doc.rust-lang.org/rust-by-example/zh/fn/closures/output_parameters.html

```
 *  正在执行任务: cargo run --package ch09_functions --bin 2_closures_output_parameters

   Compiling ch09_functions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch09_functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/2_closures_output_parameters`
这是一个：Fn
这是一个：FnMut
这是一个：FnOnce
```

#### https://doc.rust-lang.org/rust-by-example/zh/fn/closures/closure_examples/iter_any.html

```
 *  正在执行任务: cargo run --package ch09_functions --bin 2_closures_iter_any

   Compiling ch09_functions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch09_functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.49s
     Running `target/debug/2_closures_iter_any`
2 在 vec1 中：true
2 在 vec2 中：false
vec1 长度：3
vec1 的第一个元素是：1
2 在 array1 中：true
2 在 array2 中：false
```

#### https://doc.rust-lang.org/rust-by-example/zh/fn/closures/closure_examples/iter_find.html

```
 *  正在执行任务: cargo run --package ch09_functions --bin 2_closures_iter_find

   Compiling ch09_functions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch09_functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/2_closures_iter_find`
在 vec1 中查找 2：Some(2)
在 vec2 中查找 2：None
在 array1 中查找 2：Some(2)
在 array2 中查找 2：None
```

#### https://doc.rust-lang.org/rust-by-example/zh/fn/methods.html

```
 *  正在执行任务: cargo run --package ch09_functions --bin 1_methods

   Compiling ch09_functions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch09_functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/1_methods`
矩形周长：14
矩形面积：12
正在销毁 Pair(1, 2)
```

#### https://doc.rust-lang.org/rust-by-example/zh/fn/hof.html

```
 *  正在执行任务: cargo run --package ch09_functions --bin 3_hof

   Compiling ch09_functions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch09_functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/3_hof`
找出所有平方为奇数且小于 1000 的数字之和
命令式风格：256
函数式风格：256
```

#### https://doc.rust-lang.org/rust-by-example/zh/fn/diverging.html

```
 *  正在执行任务: cargo run --package ch09_functions --bin 4_diverging

   Compiling ch09_functions v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch09_functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/4_diverging`
9 以下（不包括 9）的奇数之和：16
```
