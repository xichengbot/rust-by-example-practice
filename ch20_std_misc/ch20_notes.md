# 1. 确保 ch20_std_misc 目录存在并创建 bin 目录

mkdir -p ch20_std_misc/src/bin

# 2. 批量创建按 URL 规律命名的练习文件

# 20.1 Threads 及其子节 (std_misc/threads.html, std_misc/threads/testcase_mapreduce.html)

touch ch20_std_misc/src/bin/1_threads.rs
touch ch20_std_misc/src/bin/1_threads_testcase_mapreduce.rs

# 20.2 Channels (std_misc/channels.html)

touch ch20_std_misc/src/bin/2_channels.rs

# 20.3 Path (std_misc/path.html)

touch ch20_std_misc/src/bin/3_path.rs

# 20.4 File I/O 及其子节 (std_misc/file/open.html, std_misc/file/create.html, std_misc/file/read_lines.html)

# 注意：网页上 20.4 本身没有直接的代码，只作为目录层级，但你可以保留一个作为概览

touch ch20_std_misc/src/bin/4_file.rs
touch ch20_std_misc/src/bin/4_file_open.rs
touch ch20_std_misc/src/bin/4_file_create.rs
touch ch20_std_misc/src/bin/4_file_read_lines.rs

# 20.5 Child processes 及其子节 (std_misc/process.html, std_misc/process/pipe.html, std_misc/process/wait.html)

touch ch20_std_misc/src/bin/5_process.rs
touch ch20_std_misc/src/bin/5_process_pipe.rs
touch ch20_std_misc/src/bin/5_process_wait.rs

# 20.6 Filesystem Operations (std_misc/fs.html)

touch ch20_std_misc/src/bin/6_fs.rs

# 20.7 Program arguments 及其子节 (std_misc/arg.html, std_misc/arg/matching.html)

touch ch20_std_misc/src/bin/7_arg.rs
touch ch20_std_misc/src/bin/7_arg_matching.rs

# 20.8 Foreign Function Interface (std_misc/ffi.html)

touch ch20_std_misc/src/bin/8_ffi.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch20_std_misc/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/threads.html

```
 *  正在执行任务: cargo run --package ch20_std_misc --bin 1_threads

   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.69s
     Running `target/debug/1_threads`
这是第 5 号线程
这是第 6 号线程
这是第 4 号线程
这是第 0 号线程
这是第 2 号线程
这是第 3 号线程
这是第 1 号线程
这是第 7 号线程
这是第 8 号线程
这是第 9 号线程
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/threads/testcase_mapreduce.html

```
 *  正在执行任务: cargo run --package ch20_std_misc --bin 1_threads_testcase_mapreduce

    Blocking waiting for file lock on artifact directory
   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.33s
     Running `target/debug/1_threads_testcase_mapreduce`
数据段 0 是"86967897737416471853297327050364959"
已处理段 0，结果=187
数据段 1 是"11861322575564723963297542624962850"
数据段 2 是"70856234701860851907960690014725639"
已处理段 1，结果=157
数据段 3 是"38397966707106094172783238747669219"
已处理段 2，结果=154
数据段 4 是"52380795257888236525459303330302837"
已处理段 3，结果=177
数据段 5 是"58495327135744041048897885734297812"
已处理段 4，结果=153
数据段 6 是"69920216438980873548808413720956532"
数据段 7 是"16278424637452589860345374828574668"
已处理段 6，结果=165
已处理段 7，结果=177
已处理段 5，结果=172
最终求和结果：1342
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/channels.html

```
 *  正在执行任务: cargo run --package ch20_std_misc --bin 2_channels

   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/2_channels`
线程 1 已完成
线程 2 已完成
线程 0 已完成
[Ok(0), Ok(1), Ok(2)]
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/path.html

```
 *  正在执行任务: cargo run --package ch20_std_misc --bin 3_path

   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/3_path`
新路径是 ./a/b/c/package.tgz
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/file/open.html

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/4_file_open`
拼接的绝对路径是: /Users/xichengbot/rustProjects/rust-by-example-practice/hello.txt
hello.txt 的内容：
Hello Rust!
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/file/create.html

```
 *  正在执行任务: cargo run --package ch20_std_misc --bin 4_file_create

   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/4_file_create`
成功写入 lorem_ipsum.txt
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/file/read_lines.html

```
   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/4_file_read_lines`
Hello Rust!
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/process.html

```
 *  正在执行任务: cargo run --package ch20_std_misc --bin 5_process

   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/5_process`
rustc 执行成功，标准输出为：
rustc 1.95.0 (59807616e 2026-04-14)
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/process/pipe.html

```
 *  正在执行任务: cargo run --package ch20_std_misc --bin 5_process_pipe

   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/5_process_pipe`
已将 pangram 发送给 wc
wc 的响应为：
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/process/wait.html

```
 *  正在执行任务: cargo run --package ch20_std_misc --bin 5_process_wait

   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running `target/debug/5_process_wait`
到达 main 函数末尾
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/fs.html

```
 *  正在执行任务: cargo run --package ch20_std_misc --bin 6_fs

   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.12s
     Running `target/debug/6_fs`
`mkdir a`
`echo hello > a/b.txt`
`mkdir -p a/c/d`
`touch a/c/e.txt`
`ln -s ../b.txt a/c/b.txt`
`cat a/c/b.txt`
> hello
`ls a`
> "a/b.txt"
> "a/c"
`rm a/c/e.txt`
`rmdir a/c/d`
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/arg.html

```
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/7_arg`
我的路径是 target/debug/7_arg。
我获得了 0 个参数：[]。

 ✗ ./7_arg 1 2 3
我的路径是 ./7_arg。
我获得了 3 个参数：["1", "2", "3"]。

```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/arg/matching.html

```
bin git:(main) ✗ ./7_arg_matching
我的名字是 'match_args'。试试传递一些参数吧！
➜  bin git:(main) ✗ ./7_arg_matching 40
这不是正确答案。
➜  bin git:(main) ✗ ./7_arg_matching 42
这就是正确答案！
➜  bin git:(main) ✗ ./7_arg_matching increase 1
2
➜  bin git:(main) ✗ ./7_arg_matching increase 99
100
➜  bin git:(main) ✗ ./7_arg_matching decrease 101
100
➜  bin git:(main) ✗ ./7_arg_matching decrease 100
```

#### https://doc.rust-lang.org/rust-by-example/zh/std_misc/ffi.html

```
   Compiling ch20_std_misc v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch20_std_misc)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/8_ffi`
-1+0i 的平方根是 0+1i
cos(-1+0i) = 0.5403023+0i
```
