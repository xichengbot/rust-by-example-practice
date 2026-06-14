use std::rc::Rc;

fn main() {
    let rc_examples = "Rc 示例".to_string();
    {
        println!("--- rc_a 已创建 ---");

        let rc_a: Rc<String> = Rc::new(rc_examples);
        println!("rc_a 的引用计数：{}", Rc::strong_count(&rc_a));

        {
            println!("--- rc_a 被克隆为 rc_b ---");

            let rc_b: Rc<String> = Rc::clone(&rc_a);
            println!("rc_b 的引用计数：{}", Rc::strong_count(&rc_b));
            println!("rc_a 的引用计数：{}", Rc::strong_count(&rc_a));

            // 两个 `Rc` 如果内部值相等，则它们相等
            println!("rc_a 和 rc_b 是否相等：{}", rc_a.eq(&rc_b));

            // 我们可以直接使用值的方法
            println!("rc_a 内部值的长度：{}", rc_a.len());
            println!("rc_b 的值：{}", rc_b);

            println!("--- rc_b 超出作用域被释放 ---");
        }

        println!("rc_a 的引用计数：{}", Rc::strong_count(&rc_a));

        println!("--- rc_a 超出作用域被释放 ---");
    }

    // 错误！`rc_examples` 已经被移动到 `rc_a` 中
    // 当 `rc_a` 被释放时，`rc_examples` 也会一同被释放
    // println!("rc_examples: {}", rc_examples);
    // TODO：尝试取消注释上面这行
}