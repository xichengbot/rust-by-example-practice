// 创建一个具有 `'static` 生命周期的常量。
static NUM: i32 = 18;

// 返回一个指向 `NUM` 的引用，其中 `'static` 生命周期
// 被强制转换为输入参数的生命周期。
fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

use std::fmt::Debug;

fn print_it(input: impl Debug + 'static) {
    println!("传入的 'static 值是：{:?}", input);
}

fn main() {
    {
        // 创建一个字符串字面量并打印它：
        let static_string = "我存储在只读内存中";
        println!("static_string 的值：{}", static_string);

        // 当 `static_string` 离开作用域时，该引用
        // 不能再被使用，但数据仍然保留在二进制文件中。
    }

    {
        // 创建一个整数用于 `coerce_static` 函数：
        let lifetime_num = 9;

        // 将 `NUM` 的生命周期强制转换为与 `lifetime_num` 一致：
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM：{} 仍然可以访问！", NUM);

    // i 拥有所有权且不包含任何引用，因此它是 'static 的：
    let i = 5;
    print_it(i);

    // 糟糕，&i 的生命周期仅由 main() 的作用域定义，
    // 所以它不是 'static 的：
    // print_it(&i);

}