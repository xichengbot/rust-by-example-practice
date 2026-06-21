# 1. 确保 ch21_testing 目录存在并创建 bin 目录

mkdir -p ch21_testing/src/bin

# 2. 批量创建按 URL 和小节规律命名的练习文件

# 21. 测试概览 (testing.html)

touch ch21_testing/src/bin/0_testing.rs

# 21.1 单元测试 (testing/unit_testing.html)

touch ch21_testing/src/bin/1_unit_testing.rs

# 21.2 文档测试 (testing/doc_testing.html)

# 注意：文档测试通常写在 lib.rs 中，但在 bin 文件里也可以通过特定注释体验

touch ch21_testing/src/bin/2_doc_testing.rs

# 21.3 集成测试 (testing/integration_testing.html)

# 按照 Rust 规范，真正的集成测试应该放在项目根目录的 tests/ 文件夹下，

# 但为了保持当前练习格式的统一，我们可以先放在这里作为代码记录，或者额外建一个 tests 目录。

touch ch21_testing/src/bin/3_integration_testing.rs
mkdir -p ch21_testing/tests
touch ch21_testing/tests/integration_test.rs

# 21.4 开发依赖 (testing/dev_dependencies.html)

touch ch21_testing/src/bin/4_dev_dependencies.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch21_testing/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/testing/unit_testing.html

```
 *  正在执行任务: cargo test --package ch21_testing --bin 1_unit_testing -- tests --nocapture

   Compiling ch21_testing v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch21_testing)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running unittests src/bin/1_unit_testing.rs (target/debug/deps/1_unit_testing-d62e5606f7079d95)

running 7 tests
test tests::ignored_test ... ignored
thread 'tests::test_any_panic' (6570098) panicked at ch21_testing/src/bin/1_unit_testing.rs:8:9:
除以零错误

test tests::test_add ... ok
test tests::test_add_hundred ... ok
test tests::test_divide ... ok
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:80:14
   2: _1_unit_testing::divide_non_zero_result
             at ./src/bin/1_unit_testing.rs:8:9
   3: _1_unit_testing::tests::test_any_panic
             at ./src/bin/1_unit_testing.rs:45:9
   4: _1_unit_testing::tests::test_any_panic::{{closure}}
             at ./src/bin/1_unit_testing.rs:44:24
   5: core::ops::function::FnOnce::call_once
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
   6: <fn() -> core::result::Result<(), alloc::string::String> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'tests::test_specific_panic' (6570100) panicked at ch21_testing/src/bin/1_unit_testing.rs:10:9:
除法结果为零
stack backtrace:
test tests::test_any_panic - should panic ... ok
   0: __rustc::rust_begin_unwind
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:80:14
   2: _1_unit_testing::divide_non_zero_result
             at ./src/bin/1_unit_testing.rs:10:9
   3: _1_unit_testing::tests::test_specific_panic
             at ./src/bin/1_unit_testing.rs:51:9
   4: _1_unit_testing::tests::test_specific_panic::{{closure}}
             at ./src/bin/1_unit_testing.rs:50:29
   5: core::ops::function::FnOnce::call_once
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
   6: <fn() -> core::result::Result<(), alloc::string::String> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'tests::test_specific_panic_shorthand' (6570101) panicked at ch21_testing/src/bin/1_unit_testing.rs:10:9:
除法结果为零
test tests::test_specific_panic - should panic ... ok
stack backtrace:
   0: __rustc::rust_begin_unwind
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/std/src/panicking.rs:689:5
   1: core::panicking::panic_fmt
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/panicking.rs:80:14
   2: _1_unit_testing::divide_non_zero_result
             at ./src/bin/1_unit_testing.rs:10:9
   3: _1_unit_testing::tests::test_specific_panic_shorthand
             at ./src/bin/1_unit_testing.rs:57:9
   4: _1_unit_testing::tests::test_specific_panic_shorthand::{{closure}}
             at ./src/bin/1_unit_testing.rs:56:39
   5: core::ops::function::FnOnce::call_once
             at /Users/xichengbot/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
   6: <fn() -> core::result::Result<(), alloc::string::String> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/59807616e1fa2540724bfbac14d7976d7e4a3860/library/core/src/ops/function.rs:250:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test tests::test_specific_panic_shorthand - should panic ... ok

test result: ok. 6 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

#### 运行特定测试

```
cargo test test_any_panic
cargo test panic
```

cargo test -- --ignored 来运行被忽略的测试。

#### https://doc.rust-lang.org/rust-by-example/zh/testing/doc_testing.html

#### https://doc.rust-lang.org/rust-by-example/zh/testing/integration_testing.html

#### https://doc.rust-lang.org/rust-by-example/zh/testing/dev_dependencies.html
