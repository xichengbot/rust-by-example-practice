fn main() {
    let mut _mutable_integer = 7i32;

    {
        // 通过不可变的 `_mutable_integer` 进行遮蔽
        let _mutable_integer = _mutable_integer;

        // 错误！`_mutable_integer` 在此作用域中被冻结
        // _mutable_integer = 50;
        // 修复：^ 注释掉此行

        // `_mutable_integer` 离开作用域
    }

    // 正确！`_mutable_integer` 在此作用域中未被冻结
    _mutable_integer = 3;
}