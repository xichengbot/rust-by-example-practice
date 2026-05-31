// raii.rs
fn create_box() {
    // 在堆上分配一个整数
    let _box1 = Box::new(3i32);

    // `_box1` 在此处被销毁，内存被释放
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop 正在被丢弃");
    }
}

fn main() {
    // 在堆上分配一个整数
    let _box2 = Box::new(5i32);

    // 一个嵌套的作用域：
    {
        // 在堆上分配一个整数
        let _box3 = Box::new(4i32);

        // `_box3` 在此处被销毁，内存被释放
    }

    // 创建大量的 box（仅为演示）
    // 无需手动释放内存！
    for _ in 0u32..1_000 {
        create_box();
    }

    let x = ToDrop;
    println!("创建了一个 ToDrop！");
    
    // `_box2` 在此处被销毁，内存被释放
}