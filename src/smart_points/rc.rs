use std::rc::Rc;

#[test]
fn test(){
    let one = Rc::new(1); // 创建一个 Rc 实例 one，包装整数值 1
    let another_one = one.clone(); // 克隆 Rc 实例 one，创建另一个指向同一资源的 Rc 实例 another_one
    println!("count: {}", Rc::strong_count(&another_one)); // 打印引用计数的值
}