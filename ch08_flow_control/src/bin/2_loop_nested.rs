#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("进入外层循环");

        'inner: loop {
            println!("进入内层循环");

            // 这只会中断内层循环
            // break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("这一点永远不会到达");
    }

    println!("退出外层循环");
}