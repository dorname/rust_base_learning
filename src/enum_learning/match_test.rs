fn match_test_string(x:Option<String>) -> String{
    match x{
        None => String::from("啥都没有"),
        Some(i) => i
    }
}
fn match_test_string_one(x:&str) -> String{
    match x{
        // _ => String::from("测试字符串匹配"),
        "test1" => String::from("验证"),
        //默认匹配应该放在最后
        &_ => String::from("测试字符串匹配")
    }
}
#[test]
fn test(){
    println!("{:?}",match_test_string_one("test12"));
}