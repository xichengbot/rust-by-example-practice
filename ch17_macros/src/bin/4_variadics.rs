macro_rules! calculate {
    // 单个 `eval` 的模式
    (eval $e:expr) => {
        {
            let val: usize = $e; // 强制类型为整数
            println!("{} = {}", stringify!{$e}, val);
        }
    };

    // 递归分解多个 `eval`
    (eval $e:expr, $(eval $es:expr),+) => {{
        calculate! { eval $e }
        calculate! { $(eval $es),+ }
    }};
}

fn main() {
    calculate! { // 瞧瞧！这是可变参数的 `calculate!`！
        eval 1 + 2,
        eval 3 + 4,
        eval (2 * 3) + 1
    }
}