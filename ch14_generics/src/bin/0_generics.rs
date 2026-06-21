// 具体类型 `A`
struct A;

// 定义 `Single` 类型时，首次使用 `A` 前没有 `<A>`
// 因此，`Single` 是具体类型，`A` 即上面定义的类型
struct Single(A);
//            ^ 这里是 `Single` 首次使用 `A` 类型

// 这里 `<T>` 在首次使用 `T` 之前，所以 `SingleGen` 是泛型
// 由于类型参数 `T` 是泛型，它可以是任何类型
// 包括上面定义的具体类型 `A`
struct SingleGen<T>(T);

fn main() {
    // `Single` 是具体类型，明确接受 `A`
    let _s = Single(A);

    // 创建 `SingleGen<char>` 类型的变量 `_char`
    // 并赋值为 `SingleGen('a')`
    // 这里 `SingleGen` 明确指定了类型参数
    let _char: SingleGen<char> = SingleGen('a');

    // `SingleGen` 也可以隐式指定类型参数：
    let _t    = SingleGen(A); // 使用上面定义的 `A`
    let _i32  = SingleGen(6); // 使用 `i32`
    let _char = SingleGen('a'); // 使用 `char`
}