# 1. 确保 ch11_crates 目录存在并创建 bin 目录

mkdir -p ch11_crates/src/bin

# 2. 批量创建按 URL 和小节规律命名的练习文件

# 11.1 创建库 (crates/lib.html)

# 注意：这节原本是教你写 lib.rs 的，但在我们统一的 bin 模式下，你可以把测试代码写在这里

touch ch11_crates/src/bin/1_lib.rs

# 11.2 使用库 (crates/link.html)

touch ch11_crates/src/bin/2_link.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch11_crates/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/crates/lib.html

```
rustc --crate-type=lib 1_lib.rs
➜  bin git:(main) ✗ ls
1_lib.rs      2_link.rs     lib1_lib.rlib
➜  bin git:(main) ✗

```

#### https://doc.rust-lang.org/rust-by-example/zh/crates/using_lib.html

```
rustc 2_link.rs --extern ray=lib1_lib.rlib && ./2_link
调用了 rary 的 `public_function()`
调用了 rary 的 `indirect_access()`，它
> 调用了 rary 的 `private_function()`
```
