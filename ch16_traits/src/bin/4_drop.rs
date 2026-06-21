struct Droppable {
    name: &'static str,
}

// 这个简单的 `drop` 实现添加了一个控制台打印
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> 正在释放 {}", self.name);
    }
}

fn main()  -> std::io::Result<()>  {
    let _a = Droppable { name: "a" };

    // 块 A
    {
        let _b = Droppable { name: "b" };

        // 块 B
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };

            println!("正在退出块 B");
        }
        println!("刚刚退出了块 B");

        println!("正在退出块 A");
    }
    println!("刚刚退出了块 A");

    // 可以使用 `drop` 函数手动释放变量
    drop(_a);
    // TODO ^ 试试注释掉这一行

    println!("main 函数结束");

    // `_a` 在这里**不会**被再次 `drop`，因为它已经
    // 被（手动）`drop` 过了

    // 创建新作用域来演示丢弃行为
    {
        let temp = TempFile::new("test.txt".into())?;
        println!("已创建临时文件");
        // 当 temp 超出作用域时，文件会自动清理
    }
    println!("作用域结束 - 文件应该被清理");

    // 如果需要，我们也可以手动丢弃
    let temp2 = TempFile::new("another_test.txt".into())?;
    drop(temp2); // 显式丢弃文件
    println!("手动丢弃的文件");

    Ok(())
}

use std::fs::File;
use std::path::PathBuf;

struct TempFile {
    file: File,
    path: PathBuf,
}

impl TempFile {
    fn new(path: PathBuf) -> std::io::Result<Self> {
        // 注意：File::create() 会覆盖现有文件
        let file = File::create(&path)?;

        Ok(Self { file, path })
    }
}

// When TempFile is dropped:
// 1. First, our custom drop implementation runs. The file is still open at this point,
//    but we can remove it from the filesystem by path.
// 2. Then, after our drop returns, Rust automatically drops each field,
//    so File's drop runs and closes the file handle.
impl Drop for TempFile {
    fn drop(&mut self) {
        // Note: the File is still open here — field destructors run after this method.
        if let Err(e) = std::fs::remove_file(&self.path) {
            eprintln!("删除临时文件失败：{}", e);
        }
        println!("> 已丢弃临时文件：{:?}", self.path);
        // After this method returns, Rust will drop each field (including `file`),
        // which closes the underlying file handle.
    }
}