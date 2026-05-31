// 在这里，Rust 推断出一个尽可能短的生命周期。
// 然后两个引用被强制转换为该生命周期。
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` 表示生命周期 `'a` 至少和 `'b` 一样长。
// 这里，我们接收一个 `&'a i32` 并返回一个 `&'b i32` 作为强制转换的结果。
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // 较长的生命周期

    {
        let second = 3; // 较短的生命周期

        println!("乘积是 {}", multiply(&first, &second));
        println!("{} 是第一个", choose_first(&first, &second));
    };
}