fn main() {
    // 通过类型注解，编译器得知 `elem` 的类型为 u8 
    let elem = 5u8;

    // 创建一个空向量（可增长的数组）
    let mut vec = Vec::new();
    // 此时编译器还不知道 `vec` 的具体类型，
    // 只知道它是某种类型的向量（`Vec<_>`）。

    // 将 `elem` 插入向量中
    vec.push(elem);
    // 啊哈！现在编译器知道 `vec` 是 `u8` 类型的向量（`Vec<u8>`）
    // TODO ^ 尝试注释掉 `vec.push(elem)` 这一行

    println!("{:?}", vec);
}