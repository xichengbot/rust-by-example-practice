use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // 创建指向目标文件的路径
    let path = Path::new("hello.txt");
    // 获取当前工作目录，并与相对路径联结
    if let Ok(current_dir) = env::current_dir() {
        let abs_path = current_dir.join(path);
        println!("拼接的绝对路径是: {}", abs_path.display());
    }
    let display = path.display();

    // 以只读模式打开路径，返回 `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("无法打开 {}: {}", display, why),
        Ok(file) => file,
    };

    // 将文件内容读入字符串，返回 `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("无法读取 {}: {}", display, why),
        Ok(_) => print!("{} 的内容：\n{}", display, s),
    }

    // `file` 超出作用域，"hello.txt" 文件随之关闭
}