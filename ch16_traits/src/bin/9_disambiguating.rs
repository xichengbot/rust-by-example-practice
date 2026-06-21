trait UsernameWidget {
    // 从这个小部件中获取选定的用户名
    fn get(&self) -> String;
}

trait AgeWidget {
    // 从这个小部件中获取选定的年龄
    fn get(&self) -> u8;
}

// 一个同时包含 UsernameWidget 和 AgeWidget 的表单
struct Form {
    username: String,
    age: u8,
}

impl UsernameWidget for Form {
    fn get(&self) -> String {
        self.username.clone()
    }
}

impl AgeWidget for Form {
    fn get(&self) -> u8 {
        self.age
    }
}

fn main() {
    let form = Form {
        username: "rustacean".to_owned(),
        age: 28,
    };

    // 如果你取消注释这一行，你会得到一个错误，提示
    // "找到多个 `get`"。这是因为确实存在多个
    // 名为 `get` 的方法。
    // println!("{}", form.get());
    let username1 = UsernameWidget::get(&form);
    println!("username is :{}", username1);
    let username = <Form as UsernameWidget>::get(&form);
    assert_eq!("rustacean".to_owned(), username);
    let age = <Form as AgeWidget>::get(&form);
    assert_eq!(28, age);
}