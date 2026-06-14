fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // 生成错误 1
    2 * first.parse::<i32>().unwrap() // 生成错误 2
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("第一个数的两倍是 {}", double_first(numbers));

    println!("第一个数的两倍是 {}", double_first(empty));
    // 错误 1：输入向量为空

    println!("第一个数的两倍是 {}", double_first(strings));
    // 错误 2：元素无法解析为数字
}