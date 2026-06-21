fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("整数：{:?}", copied_integer);
    println!("布尔值：{:?}", a_boolean);
    println!("单元值：{:?}", unit);

    // 编译器会对未使用的变量绑定发出警告
    // 可以通过在变量名前加下划线来消除这些警告
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // 修复：^ 在变量名前加下划线以消除警告
    // 注意：在浏览器中可能不会显示警告
}