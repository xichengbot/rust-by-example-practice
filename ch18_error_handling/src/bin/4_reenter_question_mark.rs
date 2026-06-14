use std::error;
use std::fmt;

// 将别名改为使用 `Box<dyn error::Error>`。
type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct EmptyVec;

impl fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "无效的第一个待加倍项")
    }
}

impl error::Error for EmptyVec {}

// 结构与之前相同，但不再链式处理所有的 `Result` 和 `Option`，
// 而是使用 `?` 立即获取内部值。
fn double_first(vec: Vec<&str>) -> Result<i32> {
    let first = vec.first().ok_or(EmptyVec)?;
    let parsed = first.parse::<i32>()?;
    Ok(2 * parsed)
}

fn print(result: Result<i32>) {
    match result {
        Ok(n)  => println!("第一个数的两倍是 {}", n),
        Err(e) => println!("错误：{}", e),
    }
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}