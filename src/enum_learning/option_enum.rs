pub fn option_test_plus_one(x:Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
fn option_test_plus(x:Option<i32>) -> i32{
    match x {
        None => 0,
        Some(i) => i+1
    }
}
//写一个Sting类型的模式匹配例子


#[test]
fn test(){
    // option_test_plus_one(Some(5));
    println!("test,{:?}",option_test_plus_one(Some(5)));
    println!("test,{}",option_test_plus(Some(5)));
}