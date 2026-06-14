// `NanoSecond`、`Inch` 和 `U64` 都是 `u64` 的新名称。
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // `NanoSecond` = `Inch` = `U64` = `u64`。
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    // 注意，类型别名*不会*提供额外的类型安全性，因为别名*不是*新类型
    println!("{} 纳秒 + {} 英寸 = {} 单位？",
             nanoseconds,
             inches,
             nanoseconds + inches);
}