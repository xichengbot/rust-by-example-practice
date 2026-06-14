# 1. 确保 ch18_error_handling 目录存在并创建 bin 目录

mkdir -p ch18_error_handling/src/bin

# 2. 批量创建按 URL 规律命名的练习文件

# 18.1 panic (error/panic.html)

touch ch18_error_handling/src/bin/1_panic.rs

# 18.2 Option & unwrap 及其子节 (error/option_unwrap/\*.html)

touch ch18_error_handling/src/bin/2_option_unwrap.rs
touch ch18_error_handling/src/bin/2_option_unwrap_question_mark.rs
touch ch18_error_handling/src/bin/2_option_unwrap_map.rs
touch ch18_error_handling/src/bin/2_option_unwrap_and_then.rs
touch ch18_error_handling/src/bin/2_option_unwrap_defaults.rs

# 18.3 Result 及其子节 (error/result/\*.html)

touch ch18_error_handling/src/bin/3_result.rs
touch ch18_error_handling/src/bin/3_result_map.rs
touch ch18_error_handling/src/bin/3_result_alias.rs
touch ch18_error_handling/src/bin/3_early_returns.rs
touch ch18_error_handling/src/bin/3_enter_question_mark.rs

# 18.4 Multiple error types 及其子节 (error/multiple_error_types/\*.html)

touch ch18_error_handling/src/bin/4_multiple_error_types.rs
touch ch18_error_handling/src/bin/4_option_result.rs
touch ch18_error_handling/src/bin/4_define_error_type.rs
touch ch18_error_handling/src/bin/4_boxing_errors.rs
touch ch18_error_handling/src/bin/4_reenter_question_mark.rs
touch ch18_error_handling/src/bin/4_wrap_error.rs

# 18.5 Iterating over Results (error/iter_result.html)

touch ch18_error_handling/src/bin/5_iter_result.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch18_error_handling/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/error/panic.html

```
 *  正在执行任务: cargo run --package ch18_error_handling --bin 1_panic

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/1_panic`
来点清爽的水就是我现在需要的。

thread 'main' (1443641) panicked at ch18_error_handling/src/bin/1_panic.rs:3:31:
啊啊啊啊啊！！！！
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:80:14
   2: _1_panic::drink
             at ./ch18_error_handling/src/bin/1_panic.rs:3:34
   3: _1_panic::main
             at ./ch18_error_handling/src/bin/1_panic.rs:10:5
   4: core::ops::function::FnOnce::call_once
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

 *  终端进程“cargo 'run', '--package', 'ch18_error_handling', '--bin', '1_panic'”已终止，退出代码: 101。
 *  终端将被任务重用，按任意键关闭。

```

#### https://doc.rust-lang.org/rust-by-example/zh/error/abort_unwind.html

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/1_abort_unwind`
来点清爽的水就是我现在需要的。
快吐出来！！！！
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/option_unwrap.html

```
正在执行任务: cargo run --package ch18_error_handling --bin 2_option_unwrap

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/2_option_unwrap`
水？真不错。
呸！太甜了。
没有饮料？好吧。
我超爱咖啡！！！！！

thread 'main' (1472294) panicked at ch18_error_handling/src/bin/2_option_unwrap.rs:16:24:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/option.rs:2236:5
   4: core::option::Option<T>::unwrap
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:1016:21
   5: _2_option_unwrap::drink
             at ./ch18_error_handling/src/bin/2_option_unwrap.rs:16:24
   6: _2_option_unwrap::main
             at ./ch18_error_handling/src/bin/2_option_unwrap.rs:35:5
   7: core::ops::function::FnOnce::call_once
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/option_unwrap/question_mark.html

```
   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/2_option_unwrap_question_mark`
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/option_unwrap/map.html

```
 *  正在执行任务: cargo run --package ch18_error_handling --bin 2_option_unwrap_map

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/2_option_unwrap_map`
嗯，我喜欢 Cooked(Apple)
嗯，我喜欢 Cooked(Carrot)
哎呀！这不能吃。
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/option_unwrap/and_then.html

```
 *  正在执行任务: cargo run --package ch18_error_handling --bin 2_option_unwrap_and_then

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
warning: unused variable: `a`
  --> ch18_error_handling/src/bin/2_option_unwrap_and_then.rs:40:9
   |
40 |     let a = have_recipe(food).map(have_ingredients);
   |         ^ help: if this is intentional, prefix it with an underscore: `_a`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `ch18_error_handling` (bin "2_option_unwrap_and_then") generated 1 warning (run `cargo fix --bin "2_option_unwrap_and_then" -p ch18_error_handling` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/2_option_unwrap_and_then`
哦不。我们在 Monday 没有东西吃吗？
太好了！在 Tuesday 我们可以吃到 Steak。
哦不。我们在 Wednesday 没有东西吃吗？
 *  终端将被任务重用，按任意键关闭。
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/option_unwrap/defaults.html

```
   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
warning: variant `Banana` is never constructed
 --> ch18_error_handling/src/bin/2_option_unwrap_defaults.rs:2:29
  |
2 | enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }
  |      -----                  ^^^^^^
  |      |
  |      variant in this enum
  |
  = note: `Fruit` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch18_error_handling` (bin "2_option_unwrap_defaults") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/2_option_unwrap_defaults`
第一个可用的水果：Some(Orange)
提供猕猴桃作为备选
第一个可用的水果：Some(Kiwi)
第一个可用的水果是：Apple
我的水果是：Some(Apple)
提供柠檬作为备选
第一个可用的水果是：Lemon
我的水果是：Some(Lemon)
should_be_apple 的值为：Apple
my_apple 保持不变：Some(Apple)
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/result.html

```
warning: function `multiply` is never used
 --> ch18_error_handling/src/bin/3_result.rs:1:4
  |
1 | fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
  |    ^^^^^^^^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: `ch18_error_handling` (bin "3_result") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/3_result`
10
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/result/result_map.html

```
 *  正在执行任务: cargo run --package ch18_error_handling --bin 3_result_map

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/3_result_map`
n 是 20
错误：invalid digit found in string
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/result/result_alias.html

```
 *  正在执行任务: cargo run --package ch18_error_handling --bin 3_result_alias

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/3_result_alias`
n 是 20
错误：invalid digit found in string
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/result/early_returns.html

```
 *  正在执行任务: cargo run --package ch18_error_handling --bin 3_early_returns

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/3_early_returns`
n 是 20
错误：invalid digit found in string
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/result/enter_question_mark.html

```
 *  正在执行任务: cargo run --package ch18_error_handling --bin 3_enter_question_mark

    Blocking waiting for file lock on artifact directory
   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/3_enter_question_mark`
n 是 20
错误：invalid digit found in string
 *  终端将被任务重用，按任意键关闭。
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/multiple_error_types.html

```
  正在执行任务: cargo run --package ch18_error_handling --bin 4_multiple_error_types

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/4_multiple_error_types`
第一个数的两倍是 84

thread 'main' (1686852) panicked at ch18_error_handling/src/bin/4_multiple_error_types.rs:2:29:
called `Option::unwrap()` on a `None` value
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:80:14
   2: core::panicking::panic
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:150:5
   3: core::option::unwrap_failed
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/option.rs:2236:5
   4: core::option::Option<T>::unwrap
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/option.rs:1016:21
   5: _4_multiple_error_types::double_first
             at ./ch18_error_handling/src/bin/4_multiple_error_types.rs:2:29
   6: _4_multiple_error_types::main
             at ./ch18_error_handling/src/bin/4_multiple_error_types.rs:13:45
   7: core::ops::function::FnOnce::call_once
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

 *  终端进程“cargo 'run', '--package', 'ch18_error_handling', '--bin', '4_multiple_error_types'”已终止，退出代码: 101。
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/multiple_error_types/option_result.html

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/4_option_result`
第一个数的两倍是 Ok(Some(84))
第一个数的两倍是 Ok(None)
第一个数的两倍是 Err(ParseIntError { kind: InvalidDigit })
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/multiple_error_types/define_error_type.html

```
   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/4_define_error_type`
第一个数的两倍是 84
错误：无效的第一个待加倍项
错误：无效的第一个待加倍项
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/multiple_error_types/boxing_errors.html

```
 *  正在执行任务: cargo run --package ch18_error_handling --bin 4_boxing_errors

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/4_boxing_errors`
第一个数的两倍是 84
错误：无效的第一个待加倍项
错误：invalid digit found in string
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/multiple_error_types/reenter_question_mark.html

```
 *  正在执行任务: cargo run --package ch18_error_handling --bin 4_reenter_question_mark

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running `target/debug/4_reenter_question_mark`
第一个数的两倍是 84
错误：无效的第一个待加倍项
错误：invalid digit found in string
```

#### https://doc.rust-lang.org/rust-by-example/zh/error/multiple_error_types/wrap_error.html

```
 *  正在执行任务: cargo run --package ch18_error_handling --bin 4_wrap_error

   Compiling ch18_error_handling v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch18_error_handling)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/4_wrap_error`
第一个数的两倍是 84
错误：请使用至少包含一个元素的向量
错误：提供的字符串无法解析为整数
  错误原因：invalid digit found in string

```

#### https://doc.rust-lang.org/rust-by-example/zh/error/iter_result.html

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/5_iter_result`
结果：[93, 18]
数字：[42, 93, 18]
错误：[ParseIntError { kind: InvalidDigit }, ParseIntError { kind: PosOverflow }]
结果：Err(ParseIntError { kind: InvalidDigit })
数字：[Ok(93), Ok(18)]
错误：[Err(ParseIntError { kind: InvalidDigit })]
数字：[93, 18]
错误：[ParseIntError { kind: InvalidDigit }]
```
