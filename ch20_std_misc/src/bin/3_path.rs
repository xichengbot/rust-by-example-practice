use std::path::Path;

fn main() {
    // 从 `&'static str` 创建一个 `Path`
    let path = Path::new(".");

    // `display` 方法返回一个可用于显示的结构体
    let _display = path.display();

    // `join` 使用操作系统特定的分隔符将路径与字节容器合并，
    // 并返回一个 `PathBuf`
    let mut new_path = path.join("a").join("b");

    // `push` 使用 `&Path` 扩展 `PathBuf`
    new_path.push("c");
    new_path.push("myfile.tar.gz");

    // `set_file_name` 更新 `PathBuf` 的文件名
    new_path.set_file_name("package.tgz");

    // 将 `PathBuf` 转换为字符串切片
    match new_path.to_str() {
        None => panic!("新路径不是有效的 UTF-8 序列"),
        Some(s) => println!("新路径是 {}", s),
    }
}