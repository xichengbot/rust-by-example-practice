fn main() {
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            // 注意这个 match 表达式的返回类型必须是 u32，
            // 因为 "addition" 变量的类型是 u32。
            let addition: u32 = match i%2 == 1 {
                // "i" 变量的类型是 u32，这完全没问题。
                true => i,
                // 另一方面，"continue" 表达式不返回 u32，
                // 但这仍然可以，因为它永远不会返回，
                // 因此不违反 match 表达式的类型要求。
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("9 以下（不包括 9）的奇数之和：{}", sum_odd_numbers(9));
}