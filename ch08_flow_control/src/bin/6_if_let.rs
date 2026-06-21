// 我们的示例枚举
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    // 以下都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构的含义是：如果 `let` 能将 `number` 解构为
    // `Some(i)`，则执行代码块（`{}`）。
    if let Some(i) = number {
        println!("匹配到 {:?}！", i);
    }

    // 如果需要指定匹配失败的情况，可以使用 else：
    if let Some(i) = letter {
        println!("匹配到 {:?}！", i);
    } else {
        // 解构失败。转到失败处理的情况。
        println!("没有匹配到数字。那就用一个字母吧！");
    }

    // 提供一个修改后的失败条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("匹配到 {:?}！", i);
    // 解构失败。评估 `else if` 条件，看是否应该执行替代的失败分支：
    } else if i_like_letters {
        println!("没有匹配到数字。那就用一个字母吧！");
    } else {
        // 条件判断为假。这个分支是默认情况：
        println!("我不喜欢字母。那就用个表情符号吧 :)！");
    }

    // 创建示例变量
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    // 变量 a 匹配 Foo::Bar
    if let Foo::Bar = a {
        println!("a 是 foobar");
    }

    // 变量 b 不匹配 Foo::Bar
    // 所以这里不会打印任何内容
    if let Foo::Bar = b {
        println!("b 是 foobar");
    }

    // 变量 c 匹配 Foo::Qux，它包含一个值
    // 类似于前面例子中的 Some()
    if let Foo::Qux(value) = c {
        println!("c 是 {}", value);
    }

    // `if let` 也可以进行绑定
    if let Foo::Qux(value @ 100) = c {
        println!("c 是一百");
    }

    if let Foo::Bar = a {
        println!("a is bar!");
    }

}