fn main() {
    // 声明一个变量绑定
    let a_binding;

    {
        let x = 2;

        // 初始化绑定
        a_binding = x * x;
    }

    println!("绑定：{}", a_binding);

    let another_binding;

    // 错误！使用未初始化的绑定
    // println!("另一个绑定：{}", another_binding);
    // 修复：^ 注释掉此行

    another_binding = 1;

    println!("另一个绑定：{}", another_binding);
}