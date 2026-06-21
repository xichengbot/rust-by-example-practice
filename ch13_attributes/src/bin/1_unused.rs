fn used_function() {}

// `#[allow(dead_code)]` 是一个用于禁用 `dead_code` lint 的属性
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}
// FIXME ^ 添加一个属性来抑制警告

fn main() {
    used_function();
}