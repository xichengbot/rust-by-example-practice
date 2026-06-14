use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.transpose()
}

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("第一个数的两倍是 {:?}", double_first(numbers));
    println!("第一个数的两倍是 {:?}", double_first(empty));
    println!("第一个数的两倍是 {:?}", double_first(strings));
}