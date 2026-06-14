// `find_min!` 将计算任意数量参数中的最小值。
macro_rules! find_min {
    // 基本情况：
    ($x:expr) => ($x);
    // `$x` 后面至少跟着一个 `$y,`
    ($x:expr, $($y:expr),+) => (
        // 对剩余的 `$y` 递归调用 `find_min!`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("{}", find_min!(1));
    println!("{}", find_min!(1 + 2, 2));
    println!("{}", find_min!(5, 2 * 3, 4));
}