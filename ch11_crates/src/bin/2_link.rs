// extern crate rary; // Rust 2015 版本或更早版本可能需要此声明

fn main() {
    ray::public_function();

    // 错误！`private_function` 是私有的
    //rary::private_function();

    ray::indirect_access();
}