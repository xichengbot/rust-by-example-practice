fn main() {
    let mut count = 0u32;

    println!("让我们数到无穷大！");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过本次迭代的剩余部分
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("好了，够了");

            // 退出这个循环
            break;
        }
    }
}