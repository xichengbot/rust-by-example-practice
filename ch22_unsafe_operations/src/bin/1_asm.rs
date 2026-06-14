use std::arch::asm;


extern "C" fn foo(arg: i32) -> i32 {
    println!("arg = {}", arg);
    arg * 2
}

fn call_foo(arg: i32) -> i32 {
    unsafe {
        let result;
        asm!(
            "call {}",
            // 要调用的函数指针
            in(reg) foo,
            // 第一个参数在 rdi 中
            in("rdi") arg,
            // 返回值在 rax 中
            out("rax") result,
            // 将所有不被 "C" 调用约定保留的寄存器
            // 标记为被破坏
            clobber_abi("C"),
        );
        result
    }
}


fn main() {

use std::arch::asm;

let x: u64;
unsafe {
    asm!("mov {}, 5", out(reg) x);
}
assert_eq!(x, 5);



    // 三个条目，每个四字节
    let mut name_buf = [0_u8; 12];
    // 字符串按顺序以 ASCII 格式存储在 ebx、edx、ecx 中
    // 由于 ebx 是保留寄存器，汇编需要保留其值
    // 因此我们在主要汇编代码前后执行 push 和 pop 操作
    // 64 位处理器的 64 位模式不允许对 32 位寄存器（如 ebx）进行 push/pop 操作
    // 所以我们必须使用扩展的 rbx 寄存器

    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov [rdi], ebx",
            "mov [rdi + 4], edx",
            "mov [rdi + 8], ecx",
            "pop rbx",
            // 我们使用指向数组的指针来存储值，以简化 Rust 代码
            // 虽然这会增加几条汇编指令，但更清晰地展示了汇编的工作方式
            // 相比于使用显式寄存器输出（如 `out("ecx") val`）
            // *指针本身*只是一个输入，尽管它在背后被写入
            in("rdi") name_buf.as_mut_ptr(),
            // 选择 cpuid 0，同时指定 eax 为被修改寄存器
            inout("eax") 0 => _,
            // cpuid 也会修改这些寄存器
            out("ecx") _,
            out("edx") _,
        );
    }

    let name = core::str::from_utf8(&name_buf).unwrap();
    println!("CPU 制造商 ID：{}", name);


    //

    unsafe {
        println!(" call foo of C: {}", foo(-3));
    }
}

