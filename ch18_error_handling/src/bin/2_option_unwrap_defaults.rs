#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("第一个可用的水果：{:?}", first_available_fruit);
    // first_available_fruit: Some(Orange)

    // `or` 会移动其参数。
    // 在上面的例子中，`or(orange)` 返回了 `Some`，所以 `or(apple)` 没有被调用。
    // 但是名为 `apple` 的变量无论如何都被移动了，不能再使用。
    // println!("变量 apple 被移动了，所以这行不会编译：{:?}", apple);
    // TODO：取消上面这行的注释来查看编译器错误

    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("提供猕猴桃作为备选");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("提供柠檬作为备选");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("第一个可用的水果：{:?}", first_available_fruit);
    // 提供猕猴桃作为备选
    // first_available_fruit: Some(Kiwi)

    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("第一个可用的水果是：{:?}", first_available_fruit);
    println!("我的水果是：{:?}", my_fruit);
    // first_available_fruit is: Apple
    // my_fruit is: Some(Apple)
    //println!("名为 `apple` 的变量已被移动：{:?}", apple);
    // TODO：取消上面这行的注释以查看编译器错误


    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("提供柠檬作为备选");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit
        .get_or_insert_with(get_lemon_as_fallback);
    println!("第一个可用的水果是：{:?}", first_available_fruit);
    println!("我的水果是：{:?}", my_fruit);
    // 提供柠檬作为备选
    // first_available_fruit is: Lemon
    // my_fruit is: Some(Lemon)

    // 如果 Option 已有值，它将保持不变，闭包不会被调用
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple 的值为：{:?}", should_be_apple);
    println!("my_apple 保持不变：{:?}", my_apple);
    // 输出如下。注意闭包 `get_lemon_as_fallback` 并未被调用
    // should_be_apple is: Apple
    // my_apple is unchanged: Some(Apple)

    
}