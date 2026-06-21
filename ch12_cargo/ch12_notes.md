# 1. 确保 ch12_cargo 目录存在并创建 bin 目录

mkdir -p ch12_cargo/src/bin

# 2. 批量创建按 URL 和小节规律命名的练习文件

# 12. Cargo 概览 (cargo.html)

touch ch12_cargo/src/bin/0_cargo.rs

# 12.1 依赖 (cargo/deps.html)

touch ch12_cargo/src/bin/1_deps.rs

# 12.2 约定 (cargo/conventions.html)

touch ch12_cargo/src/bin/2_conventions.rs

# 12.3 测试 (cargo/test.html)

touch ch12_cargo/src/bin/3_test.rs

# 12.4 构建脚本 (cargo/build_scripts.html)

touch ch12_cargo/src/bin/4_build_scripts.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch12_cargo/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/cargo/test.html

```
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/bin/3_test.rs (target/debug/deps/3_test-79219adca4ac6a56)

running 2 tests
test tests::test_file_also ... ok
test tests::test_file ... ok

```
