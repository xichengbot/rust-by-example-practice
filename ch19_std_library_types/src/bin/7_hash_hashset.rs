use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // 如果集合中已存在该值，`HashSet::insert()` 将返回 false。
    // assert!(b.insert(4), "值 4 已存在于集合 B 中！");
    // 修复：^ 注释掉此行

    b.insert(5);

    // 如果集合的元素类型实现了 `Debug` 特征，
    // 那么该集合也会实现 `Debug` 特征。
    // 通常会以 `[elem1, elem2, ...]` 的格式打印其元素
    println!("A：{:?}", a);
    println!("B：{:?}", b);

    // 以任意顺序打印 [1, 2, 3, 4, 5]
    println!("并集：{:?}", a.union(&b).collect::<Vec<&i32>>());

    // 这里应该打印 [1]
    println!("差集：{:?}", a.difference(&b).collect::<Vec<&i32>>());

    // 以任意顺序打印 [2, 3, 4]
    println!("交集：{:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // 打印 [1, 5]
    println!("对称差：{:?}",
             a.symmetric_difference(&b).collect::<Vec<&i32>>());
}