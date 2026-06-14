struct Droppable {
    name: &'static str,
}

// 这个简单的 `drop` 实现添加了一个控制台打印
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> 正在释放 {}", self.name);
    }
}

fn main() {
    let _a: Droppable = Droppable { name: "a" };

    // 块 A
    {
        let _b = Droppable { name: "b" };

        // 块 B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("正在退出块 B");
        }
        println!("刚刚退出了块 B");

        println!("正在退出块 A");
    }
    println!("刚刚退出了块 A");

    // 可以使用 `drop` 函数手动释放变量
    drop(_a);
    // TODO ^ 试试注释掉这一行

    println!("main 函数结束");

    // `_a` 在这里**不会**被再次 `drop`，因为它已经
    // 被（手动）`drop` 过了
}