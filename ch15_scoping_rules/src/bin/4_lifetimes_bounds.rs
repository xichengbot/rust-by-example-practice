use std::fmt::Debug; // 用于约束的 trait。

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
// `Ref` 包含一个指向泛型类型 `T` 的引用，`T` 具有一个 `Ref` 未知的
// 生命周期 `'a`。`T` 受到约束，使得 `T` 中的任何*引用*必须比 `'a` 存活更久。
// 此外，`Ref` 的生命周期不能超过 `'a`。

// 一个使用 `Debug` trait 进行打印的泛型函数。
fn print<T>(t: T) where
    T: Debug {
    println!("`print`: t 是 {:?}", t);
}

// 这里取了一个 `T` 的引用，其中 `T` 实现了
// `Debug` 并且 `T` 中的所有*引用*都比 `'a` 存活更久。
// 此外，`'a` 必须比函数存活更久。
fn print_ref<'a, T>(t: &'a T) where
    T: Debug + 'a {
    println!("`print_ref`: t 是 {:?}", t);
}

fn main() {
    let x = 7;
    let ref_x = Ref(&x);

    print_ref(&ref_x);
    print(ref_x);
}