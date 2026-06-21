use std::process::Command;

fn main() {
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("执行进程失败：{}", e)
    });

    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc 执行成功，标准输出为：\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc 执行失败，标准错误输出为：\n{}", s);
    }
}