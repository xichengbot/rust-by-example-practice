use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// 这里需要一个 `where` 子句：否则就必须将其表达为 `T: Debug` 或
// 使用另一种间接方法，
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // 我们需要 `Option<T>: Debug` 作为我们的约束，因为这是
    // 正在被打印的内容。否则就会使用错误的约束。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}