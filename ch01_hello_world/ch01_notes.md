## vs code安装核心扩展 (Extensions)

打开 VS Code，点击左侧边栏的扩展图标（或按 Cmd + Shift + X），依次搜索并安装以下插件：

1. rust-analyzer (必装)：这是 Rust 开发的核心，提供代码补全、转到定义和类型提示。

2. CodeLLDB (必装)：用于在 macOS 上调试 Rust 代码，支持断点和变量查看。

3. Even Better TOML：为 Cargo.toml 文件提供语法支持和格式化。

4. Error Lens (强烈推荐)：它会将编译错误直接显示在代码行末尾，不需要把鼠标悬停在红线上看报错，非常适合练习《Rust by Example》。

5. One Dark Pro, 非常好看的 VSCode 主题

## create workspace

```
mkdir rust-by-example-practice
cd rust-by-example-practice
touch Cargo.toml
```

## configuration of Cargo.toml

```
[workspace]
# 告诉 Cargo 哪些文件夹是子项目
members = [
    "ch01_hello_world",
]
# 共享编译优化配置
resolver = "2"
```

## create sub project

```
cargo new ch01_hello_world
```

## run the sub project

```
cargo run -p ch01_hello_world
```

## output

```
cargo run ch01_hello_world
warning: field `0` is never read
  --> ch01_hello_world/src/main.rs:25:13
   |
25 | struct Deep(Structure);
   |        ---- ^^^^^^^^^
   |        |
   |        field in this struct
   |
   = help: consider removing this field
   = note: `Deep` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: fields `name` and `age` are never read
  --> ch01_hello_world/src/main.rs:91:5
   |
90 | struct Person<'a> {
   |        ------ fields in this struct
91 |     name: &'a str,
   |     ^^^^
92 |     age: u8
   |     ^^^
   |
   = note: `Person` has a derived impl for the trait `Debug`, but this is intentionally ignored during dead code analysis

warning: `ch01_hello_world` (bin "ch01_hello_world") generated 2 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ch01_hello_world ch01_hello_world`
Hello, world!
I'm a Rustacean!!
365 days
Alice, this is Bob. Bob, this is Alice
the quick brown fox jumps over the lazy dog
Base 10:               69420
Base 2 (binary):       10000111100101100
Base 8 (octal):        207454
Base 16 (hexadecimal): 10f2c
    1
00001
10000
00001
10000000000000000000
My name is Bond, james Bond
  1.2
Now Structure(3) will print!
Display: 3
Now Deep(Structure(7)) will print!
Person {
    name: "Peter",
    age: 27,
}
```

## generate docs

```
 cargo doc --no-deps --open
```
