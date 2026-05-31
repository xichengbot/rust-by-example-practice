// 此函数获取堆分配内存的所有权
fn destroy_box(c: Box<i32>) {
    println!("正在销毁一个包含 {} 的 box", c);

    // `c` 被销毁，内存被释放
}

fn main() {
    // **栈**分配的整数
    let x = 5u32;

    // 将 `x` **复制**到 `y` - 没有资源被移动
    let y = x;

    // 两个值可以独立使用
    println!("x 是 {}，y 是 {}", x, y);

    // `a` 是指向**堆**分配整数的指针
    let a = Box::new(5i32);

    println!("a 包含：{}", a);

    // 将 `a` **移动**到 `b`
    let b = a;
    // `a` 的指针地址（而非数据）被复制到 `b`
    // 现在两者都指向同一块堆分配的数据
    // 但 `b` 现在拥有它

    // 错误！`a` 不再拥有堆内存，因此无法访问数据
    // println!("a 包含：{}", a);
    // TODO ^ 尝试取消此行注释

    // 此函数从 `b` 获取堆分配内存的所有权
    destroy_box(b);

    // 此时堆内存已被释放，这个操作会导致解引用已释放的内存
    // 但编译器禁止这样做
    // 错误！原因与前面的错误相同
    // println!("b 包含：{}", b);
    // TODO ^ 尝试取消此行注释

    let immutable_box = Box::new(5u32);

    println!("immutable_box 包含 {}", immutable_box);

    // 可变性错误
    //*immutable_box = 4;

    // *移动*盒子，改变所有权（和可变性）
    let mut mutable_box = immutable_box;

    println!("mutable_box 包含 {}", mutable_box);

    // 修改盒子的内容
    *mutable_box = 4;

    println!("mutable_box 现在包含 {}", mutable_box);

    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    // 错误！无法从实现了 `Drop` 特质的类型中移动
    //impl Drop for Person {
    //    fn drop(&mut self) {
    //        println!("Dropping the person struct {:?}", self)
    //    }
    //}
    // TODO ^ 尝试取消注释这些行

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` 从 person 中被移出，但 `age` 只是被引用
    let Person { name, ref age } = person;

    println!("此人的年龄是 {}", age);

    println!("此人的姓名是 {}", name);

    // 错误！借用部分移动的值：`person` 发生部分移动
    //println!("person 结构体是 {:?}", person);

    // `person` 不能使用，但 `person.age` 可以使用，因为它没有被移动
    println!("从 person 结构体中获取的年龄是 {}", person.age);

    
}