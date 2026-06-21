use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "很抱歉，无法接通您拨打的电话。
            请挂机后重试。",
        "645-7689" => "您好，这里是 Awesome 先生的披萨店。我是 Fred。
            请问今天您想点些什么？",
        _ => "嗨！请问是哪位？"
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    // 接受一个引用并返回 Option<&V>
    match contacts.get(&"Daniel") {
        Some(&number) => println!("正在呼叫 Daniel：{}", call(number)),
        _ => println!("没有 Daniel 的电话号码。"),
    }

    // 如果插入的值是新的，`HashMap::insert()` 返回 `None`
    // 否则返回 `Some(value)`
    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("正在呼叫 Ashley：{}", call(number)),
        _ => println!("没有 Ashley 的电话号码。"),
    }

    contacts.remove(&"Ashley");

    // `HashMap::iter()` 返回一个迭代器，该迭代器以任意顺序生成
    // (&'a key, &'a value) 键值对。
    for (contact, &number) in contacts.iter() {
        println!("正在呼叫{}：{}", contact, call(number));
    }
}