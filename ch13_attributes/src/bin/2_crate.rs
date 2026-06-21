// 这个 crate 是一个库
#![crate_type = "lib"]
// 这个库的名称是 "rary"
#![crate_name = "rary"]

pub fn public_function() {
    println!("调用了 rary 的 `public_function()`");
}

fn private_function() {
    println!("调用了 rary 的 `private_function()`");
}

pub fn indirect_access() {
    print!("调用了 rary 的 `indirect_access()`，它\n> ");

    private_function();
}