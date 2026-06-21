// A unit struct without resources
// Note: Copy requires Clone, so we must derive both
#[derive(Debug, Clone, Copy)]
struct Unit;

// A tuple struct with resources that implements the `Clone` trait
// This CANNOT be Copy because Box<T> is not Copy
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    // 实例化 `Unit`
    let unit = Unit;
    // Copy `Unit` - this is an implicit copy, not a move!
    // Because Unit implements Copy, the value is duplicated automatically
    let copied_unit = unit;

    // 两个 `Unit` 可以独立使用
    println!("原始: {:?}", unit);
    println!("复制: {:?}", copied_unit);

    // 实例化 `Pair`
    let pair = Pair(Box::new(1), Box::new(2));
    println!("原始: {:?}", pair);

    // Move `pair` into `moved_pair`, moves resources
    // Pair does not implement Copy, so this is a move
    let moved_pair = pair;
    println!("已移动: {:?}", moved_pair);

    // 错误！`pair` 已失去其资源
    //println!("原始: {:?}", pair);
    // TODO ^ 尝试取消注释此行

    // Clone `moved_pair` into `cloned_pair` (resources are included)
    // Unlike Copy, Clone is explicit - we must call .clone()
    let cloned_pair = moved_pair.clone();
    // 使用 std::mem::drop 丢弃已移动的原始对
    drop(moved_pair);

    // 错误！`moved_pair` 已被丢弃
    //println!("已移动并丢弃: {:?}", moved_pair);
    // TODO ^ 尝试取消注释此行

    // .clone() 的结果仍然可以使用！
    println!("克隆: {:?}", cloned_pair);
}