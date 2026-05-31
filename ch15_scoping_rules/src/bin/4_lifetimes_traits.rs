// 一个带有生命周期注解的结构体。
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// 为 impl 添加生命周期注解。
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,//它的生命周期是'static，这里发生了静态提升
        }
    }
}

fn main() {
    let b: Borrowed = Default::default();
    println!("b 是 {:?}", b);
}