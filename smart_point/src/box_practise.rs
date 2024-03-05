#[test]
fn test(){
    let boxed_integer = Box::new(5);
    println!("测试：{:?}",*boxed_integer);
}