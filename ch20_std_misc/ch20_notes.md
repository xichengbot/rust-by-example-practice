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
