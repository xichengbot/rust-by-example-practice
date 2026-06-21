// 实现打印标记 `{:?}` 的 trait。
use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// 泛型 `T` 必须实现 `Debug`。无论 `T` 是什么类型，
// 这个函数都能正常工作。
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` 必须实现 `HasArea`。任何满足这个约束的类型
// 都可以访问 `HasArea` 的 `area` 方法。
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("面积：{}", area(&rectangle));

    // print_debug(&_triangle);
    // println!("面积：{}", area(&_triangle));
    // ^ TODO：尝试取消这些注释。
    // | 错误：未实现 `Debug` 或 `HasArea`。
}