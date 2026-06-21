fn main() {
    // （所有的类型注解都不是必须的）
    // 一个指向只读内存中分配的字符串的引用
    let pangram: &'static str = "敏捷的棕色狐狸跳过懒惰的狗狗";
    println!("全字母句：{}", pangram);

    // 反向遍历单词，不会分配新的字符串
    println!("单词逆序");
    for word in pangram.split_whitespace().rev() {
        println!("> {}", word);
    }

    // 将字符复制到向量中，排序并去重
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();

    // 创建一个空的、可增长的 `String`
    let mut string = String::new();
    for c in chars {
        // 在字符串末尾插入一个字符
        string.push(c);
        // 在字符串末尾追加另一个字符串
        string.push_str(", ");
    }

    // 修剪后的字符串是原始字符串的一个切片，因此不会进行新的内存分配
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    println!("使用的字符：{}", trimmed_str);

    // 在堆上分配一个字符串
    let alice = String::from("我喜欢狗");
    // 分配新的内存并在其中存储修改后的字符串
    let bob: String = alice.replace("狗", "猫");

    println!("爱丽丝说：{}", alice);
    println!("鲍勃说：{}", bob);

    // ---

    // 你可以使用转义来通过十六进制值写入字节...
    let byte_escape = "我正在写 \x52\x75\x73\x74！";
    println!("你在做什么\x3F（\\x3F 表示 ?）{}", byte_escape);

    // ...或 Unicode 码点。
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"双线大写 R\"";

    println!("Unicode 字符 {} (U+211D) 被称为 {}",
                unicode_codepoint, character_name );


    let long_string = "字符串字面值
                        可以跨越多行。
                        这里的换行和缩进 ->\
                        <- 也可以被转义！";
    println!("{}", long_string);


    // ---
    let raw_str = r"转义在这里不起作用：\x3F \u{211D}";
    println!("{}", raw_str);

    // 如果你需要在原始字符串中使用引号，可以添加一对 #
    let quotes = r#"然后我说："无处可逃！""#;
    println!("{}", quotes);

    // 如果你需要在字符串中使用 "#，只需在分隔符中使用更多的 #。
    // 你最多可以使用 255 个 #。
    let longer_delimiter = r###"一个包含 "# 的字符串。甚至还有 "##！"###;
    println!("{}", longer_delimiter);

    // ---
    
}