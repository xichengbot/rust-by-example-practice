fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn main() {
    println!("找出所有平方为奇数且小于 1000 的数字之和");
    let upper = 1000;

    // 命令式方法
    // 声明累加器变量
    let mut acc = 0;
    // 迭代：从 0, 1, 2, ... 到无穷大
    for n in 0.. {
        // 计算数字的平方
        let n_squared = n * n;

        if n_squared >= upper {
            // 如果超过上限则跳出循环
            break;
        } else if is_odd(n_squared) {
            // 如果是奇数，则累加值
            acc += n;
        }
    }
    println!("命令式风格：{}", acc);

    // 函数式方法
    let sum: u32 =
        (0..).take_while(|&n| n * n < upper) // 小于上限
             .filter(|&n| is_odd(n * n))     // 筛选奇数
             .sum();                         // 求和
    println!("函数式风格：{}", sum);
}