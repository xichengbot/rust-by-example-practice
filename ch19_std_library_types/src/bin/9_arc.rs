use std::time::Duration;
use std::sync::Arc;
use std::thread;

fn main() {
    // 在这个变量声明中指定了它的值。
    let apple = Arc::new("同一个苹果");

    for _ in 0..10 {
        // 这里没有指定值，因为它是指向堆内存中引用的指针。
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            // 由于使用了 Arc，可以使用 Arc 变量指针所指向的值来生成线程。
            println!("{:?}", apple);
        });
    }

    // 确保所有 Arc 实例都从生成的线程中打印出来。
    thread::sleep(Duration::from_secs(1));
}