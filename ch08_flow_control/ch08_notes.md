# 1. 确保 ch08_flow_control 目录存在并创建 bin 目录

mkdir -p ch08_flow_control/src/bin

# 2. 批量创建按 URL 和小节规律命名的练习文件

# 8.1 if/else (flow_control/if_else.html)

touch ch08_flow_control/src/bin/1_if_else.rs

# 8.2 loop 及其子节 (flow_control/loop.html, loop/nested.html, loop/return.html)

touch ch08_flow_control/src/bin/2_loop.rs
touch ch08_flow_control/src/bin/2_loop_nested.rs
touch ch08_flow_control/src/bin/2_loop_return.rs

# 8.3 while (flow_control/while.html)

touch ch08_flow_control/src/bin/3_while.rs

# 8.4 for 和区间 (flow_control/for.html)

touch ch08_flow_control/src/bin/4_for.rs

# 8.5 match 及其子节 (flow_control/match.html)

touch ch08_flow_control/src/bin/5_match.rs

# 8.5.1 解构 (Destructuring)

touch ch08_flow_control/src/bin/5_match_destructuring_tuple.rs
touch ch08_flow_control/src/bin/5_match_destructuring_slice.rs
touch ch08_flow_control/src/bin/5_match_destructuring_enum.rs
touch ch08_flow_control/src/bin/5_match_destructuring_pointers.rs
touch ch08_flow_control/src/bin/5_match_destructuring_struct.rs

# 8.5.2 卫语句 (Guards)

touch ch08_flow_control/src/bin/5_match_guard.rs

# 8.5.3 绑定 (Binding)

touch ch08_flow_control/src/bin/5_match_binding.rs

# 8.6 if let (flow_control/if_let.html)

touch ch08_flow_control/src/bin/6_if_let.rs

# 8.7 let-else (flow_control/let_else.html)

touch ch08_flow_control/src/bin/7_let_else.rs

# 8.8 while let (flow_control/while_let.html)

touch ch08_flow_control/src/bin/8_while_let.rs

# 3. 删除默认的 main.rs 以避免 Cargo 运行冲突（可选但推荐）

rm -f ch08_flow_control/src/main.rs

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/if_else.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 1_if_else

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/1_if_else`
5 是正数，是一个小数字，扩大十倍
5 -> 50
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/loop.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 2_loop

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/2_loop`
让我们数到无穷大！
1
2
three
4
5
好了，够了
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/loop/nested.html

```
   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.23s
     Running `target/debug/2_loop_nested`
进入外层循环
进入内层循环
退出外层循环
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/loop/return.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 2_loop_return

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/2_loop_return`
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/while.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 3_while

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.21s
     Running `target/debug/3_while`
1
2
fizz
4
buzz
fizz
7
8
fizz
buzz
11
fizz
13
14
fizzbuzz
16
17
fizz
19
buzz
fizz
22
23
fizz
buzz
26
fizz
28
29
fizzbuzz
31
32
fizz
34
buzz
fizz
37
38
fizz
buzz
41
fizz
43
44
fizzbuzz
46
47
fizz
49
buzz
fizz
52
53
fizz
buzz
56
fizz
58
59
fizzbuzz
61
62
fizz
64
buzz
fizz
67
68
fizz
buzz
71
fizz
73
74
fizzbuzz
76
77
fizz
79
buzz
fizz
82
83
fizz
buzz
86
fizz
88
89
fizzbuzz
91
92
fizz
94
buzz
fizz
97
98
fizz
buzz
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/for.html

```
   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/4_for`
1
2
fizz
4
buzz
fizz
7
8
fizz
buzz
11
fizz
13
14
fizzbuzz
16
17
fizz
19
buzz
fizz
22
23
fizz
buzz
26
fizz
28
29
fizzbuzz
31
32
fizz
34
buzz
fizz
37
38
fizz
buzz
41
fizz
43
44
fizzbuzz
46
47
fizz
49
buzz
fizz
52
53
fizz
buzz
56
fizz
58
59
fizzbuzz
61
62
fizz
64
buzz
fizz
67
68
fizz
buzz
71
fizz
73
74
fizzbuzz
76
77
fizz
79
buzz
fizz
82
83
fizz
buzz
86
fizz
88
89
fizzbuzz
91
92
fizz
94
buzz
fizz
97
98
fizz
buzz
101
你好 Bob
你好 Frank
我们中间有一个 Rustacean！
names: ["Bob", "Frank", "Ferris"]
你好 Bob
你好 Frank
我们中间有一个 Rustacean！
names: ["你好", "你好", "我们中间有一个 Rustacean！"]
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/match.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 5_match

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/5_match`
告诉我关于 13 的信息
一个青少年
true -> 1
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/match/destructuring/destructure_tuple.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 5_match_destructuring_tuple

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/5_match_destructuring_tuple`
告诉我关于 (0, -2, 3) 的信息
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/match/destructuring/destructure_slice.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 5_match_destructuring_slice

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/5_match_destructuring_slice`
array[0] = 1，array[2] = 6，array[1] 被忽略了
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/match/destructuring/destructure_enum.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 5_match_destructuring_enum

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/5_match_destructuring_enum`
这是什么颜色？
红：122，绿：17，蓝：40！
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/match/destructuring/destructure_pointers.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 5_match_destructuring_pointers

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/5_match_destructuring_pointers`
通过解构获得的值：4
通过解引用获得的值：4
获得了一个值的引用：5
我们加了 10。`mut_value`：16
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/match/destructuring/destructure_structures.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 5_match_destructuring_struct

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running `target/debug/5_match_destructuring_struct`
x 的第一个元素是 1，b = 2，y = 3
外部：x0 = (1, 2)，y0 = 3
嵌套：nested_x = (1, 2)，nested_y = 3
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/match/guard.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 5_match_guard

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/5_match_guard`
35°C 高于 30°C
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/match/binding.html

```
   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
warning: unreachable pattern
  --> ch08_flow_control/src/bin/5_match_binding.rs:28:14
   |
28 |         n @ (1 | 7 | 15 | 13) => println!("我是 @@@@ {:?} 岁的青少年", n),
   |              ^^^^^^^^^^^^^^^ no value can reach this
   |
note: multiple earlier patterns match some of the same values
  --> ch08_flow_control/src/bin/5_match_binding.rs:28:14
   |
25 |         n @ 1  ..= 12 => println!("我是 {:?} 岁的儿童", n),
   |             --------- matches some of the same values
26 |         n @ 13 ..= 19 => println!("我是 {:?} 岁的青少年", n),
   |             --------- matches some of the same values
27 |         // A similar binding can be done when matching several values.
28 |         n @ (1 | 7 | 15 | 13) => println!("我是 @@@@ {:?} 岁的青少年", n),
   |              ^^^^^^^^^^^^^^^ collectively making this unreachable
   = note: `#[warn(unreachable_patterns)]` (part of `#[warn(unused)]`) on by default

warning: `ch08_flow_control` (bin "5_match_binding") generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/5_match_binding`
告诉我你是什么类型的人
我是 15 岁的青少年
答案是：42！
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/if_let.html

```
   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
warning: unused variable: `value`
  --> ch08_flow_control/src/bin/6_if_let.rs:64:21
   |
64 |     if let Foo::Qux(value @ 100) = c {
   |                     ^^^^^ help: if this is intentional, prefix it with an underscore: `_value`
   |
   = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `ch08_flow_control` (bin "6_if_let") generated 1 warning (run `cargo fix --bin "6_if_let" -p ch08_flow_control` to apply 1 suggestion)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/6_if_let`
匹配到 7！
没有匹配到数字。那就用一个字母吧！
我不喜欢字母。那就用个表情符号吧 :)！
a 是 foobar
c 是 100
c 是一百
a is bar!
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/let_else.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 7_let_else

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running `target/debug/7_let_else`
```

#### https://doc.rust-lang.org/rust-by-example/zh/flow_control/while_let.html

```
 *  正在执行任务: cargo run --package ch08_flow_control --bin 8_while_let

   Compiling ch08_flow_control v0.1.0 (/Users/xichengbot/rustProjects/rust-by-example-practice/ch08_flow_control)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/8_while_let`
`i` 是 `0`。再试一次。
`i` 是 `1`。再试一次。
`i` 是 `2`。再试一次。
`i` 是 `3`。再试一次。
`i` 是 `4`。再试一次。
`i` 是 `5`。再试一次。
`i` 是 `6`。再试一次。
`i` 是 `7`。再试一次。
`i` 是 `8`。再试一次。
`i` 是 `9`。再试一次。
大于 9，退出！
```
