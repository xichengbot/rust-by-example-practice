// 定义一个函数，它接受一个由 `Fn` 约束的泛型参数 `F`，并调用它
fn call_me<F: Fn()>(f: F) {
    f();
}

fn call_me1<F: Fn()>(f: F) {
    f();
}


// 定义一个满足 `Fn` 约束的包装函数
fn function() {
    println!("我是函数！");
}

fn main() {
    // 定义一个满足 `Fn` 约束的闭包
    let closure = || println!("我是闭包！");

    call_me(closure);
    call_me(function);
}