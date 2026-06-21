use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // 第一个参数是用于调用程序的路径
    println!("我的路径是 {}。", args[0]);

    // 其余参数是传递的命令行参数
    // 像这样调用程序：
    //   $ ./args arg1 arg2
    println!("我获得了 {:?} 个参数：{:?}。", args.len() - 1, &args[1..]);
}