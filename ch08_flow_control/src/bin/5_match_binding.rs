// 一个返回 `u32` 的 `age` 函数。
fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}



fn main() {
    println!("告诉我你是什么类型的人");

    match age() {
        0             => println!("我还没有过第一个生日"),
        // 我们可以直接 `match` 1 ..= 12，但那样的话如何确定
        // 孩子的确切年龄呢？
        // 可以 `match` n 并使用 `if` 守卫，但这并不是
        // 穷尽性检查。
        // （虽然在这种情况下无关紧要，因为底部有一个
        // "兜底" 模式）
        // 因此，将 1 ..= 12 序列绑定到 `n`。
        // 现在可以报告年龄了。
        n @ 1  ..= 12 => println!("我是 {:?} 岁的儿童", n),
        n @ 13 ..= 19 => println!("我是 {:?} 岁的青少年", n),
        // A similar binding can be done when matching several values.
        n @ (1 | 7 | 15 | 13) => println!("我是 @@@@ {:?} 岁的青少年", n),
        // 没有绑定。直接返回结果。
        n             => println!("我是 {:?} 岁的成年人", n),
    }

     match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        // Could also use `Some(42)` and print `"The Answer: 42!"`
        // but that would require changing `42` in 2 spots should
        // you ever wish to change it.
        // Could also use `Some(n) if n == 42` and print `"The Answer: {n}!"`
        // but that would not contribute to exhaustiveness checks.
        // (Although in this case that would not matter since
        // the next arm is a "catch-all" pattern)
        Some(n @ 42) => println!("答案是：{}！", n),
        // 匹配任何其他数字
        Some(n)      => println!("不感兴趣... {}", n),
        // 匹配其他任何情况（`None` 变体）
        _            => (),
    }
    
}