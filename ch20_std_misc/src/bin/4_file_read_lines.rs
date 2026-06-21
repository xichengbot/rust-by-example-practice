use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // 文件 hosts.txt 必须存在于当前路径下
    if let Ok(lines) = read_lines("./hello.txt") {
        // 消耗迭代器，返回一个（可选的）String
        for line in lines.map_while(Result::ok) {
            println!("{}", line);
        }
    }
}

// 输出被包装在 Result 中以便于错误匹配。
// 返回一个指向文件行读取器的迭代器。
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}