//! 第一章的练习  
//! 行注释：从该行开始//，一直到该行末尾。  
//! 块注释：用方括号括起来/* ... */，可以跨越多行。  
//! 
//! 

use std::fmt;

#[derive(Debug)]
struct Structure(i32);

impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}


#[derive(Debug)]
struct Deep(Structure);


/// 这是main函数
fn main() {
    // 打印
    println!("Hello, world!");
    println!("I'm a Rustacean!!");

    println!("{} days", 365);

    // 使用数字占位符
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // 使用命名参数
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
    // 不同进制打印
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("{number:>5}", number=1);
    println!("{number:0>5}", number=1); // 00001
    println!("{number:0<5}", number=1); // 10000
    println!("{number:0>width$}", number=1, width=5);


    // let num = format!("{number:0<20}", number=1).as_str();
    let num = format!("{number:0<20}", number=1);
    println!("{}", num);
    
    println!("My name is {0}, {1} {0}", "Bond", "james");

    // #[allow(dead_code)] // disable `dead_code` which warn against unused module
    // struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.2;
    let width: usize = 5;
    println!("{number:>width$}");

    println!("Now {:?} will print!", Structure(3));
    println!("Display: {}", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);

}


#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}


