fn main() {
    let outer_var = 42;

    // 常规函数无法引用外部环境的变量
    //fn function(i: i32) -> i32 { i + outer_var }
    // TODO：取消上面这行的注释，查看编译器错误
    // 编译器会建议我们定义一个闭包来替代

    // 闭包是匿名的，这里我们将它们绑定到引用
    // 注解与函数注解相同，但是可选的
    // 包裹函数体的 `{}` 也是可选的
    // 这些无名函数被赋值给适当命名的变量
    let closure_annotated = |i: i32| -> i32 { i + outer_var };
    let closure_inferred  = |i     |          i + outer_var  ;

    // 调用闭包
    println!("closure_annotated：{}", closure_annotated(1));
    println!("closure_inferred：{}", closure_inferred(1));
    // 闭包类型一旦被推断，就不能再用其他类型重新推断。
    //println!("不能用其他类型重用 closure_inferred：{}", closure_inferred(42i64));
    // TODO：取消上面这行的注释，观察编译器错误。

    // 一个无参数并返回 `i32` 的闭包。
    // 返回类型是推断的。
    let one = || 1;
    println!("返回 1 的闭包：{}", one());

}