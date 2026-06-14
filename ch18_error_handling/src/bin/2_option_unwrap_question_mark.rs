struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {

    // 如果存在，获取此人工作电话号码的区号。
    fn work_phone_area_code(&self) -> Option<u8> {
        // 如果没有 `?` 运算符，这将需要许多嵌套的 `match` 语句。
        // 这将需要更多的代码 - 试着自己写一下，看看哪种方式
        // 更容易。
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));

    let a = next_birthday(None);
    print!("a is : {:?}", a);

}


fn next_birthday(current_age: Option<u8>) -> Option<String> {
    // 如果 `current_age` 是 `None`，这将返回 `None`。
    // 如果 `current_age` 是 `Some`，内部的 `u8` 值加 1
    // 后被赋值给 `next_age`
    let next_age: u8 = current_age? + 1;
    Some(format!("明年我将会 {} 岁", next_age))
}