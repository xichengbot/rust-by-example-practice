fn main() {
    // 带后缀的字面值，其类型在初始化时确定
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面值，其类型取决于使用方式
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回变量的字节大小
    println!("`x` 的字节大小：{}", std::mem::size_of_val(&x));
    println!("`y` 的字节大小：{}", std::mem::size_of_val(&y));
    println!("`z` 的字节大小：{}", std::mem::size_of_val(&z));
    println!("`i` 的字节大小：{}", std::mem::size_of_val(&i));
    println!("`f` 的字节大小：{}", std::mem::size_of_val(&f));
}