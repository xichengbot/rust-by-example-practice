/// 第一行是函数的简要描述。
///
/// 接下来的几行是详细文档。代码块以三个反引号开始，
/// 并隐含了 `fn main()` 函数和 `extern crate <cratename>` 声明。
/// 假设我们正在测试 `playground` crate：
///
/// ```
/// let result = playground::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 文档注释通常包含"示例"、"异常"和"错误"等部分。
///
/// 下面的函数用于两数相除。
///
/// # 示例
///
/// ```
/// let result = playground::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # 异常
///
/// 当第二个参数为零时，函数会触发异常。
///
/// ```rust,should_panic
/// // 除以零会触发异常
/// playground::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("除以零错误");
    }

    a / b
}