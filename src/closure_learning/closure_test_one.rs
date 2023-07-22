#[test]
fn test(){
    let mut span = 6;
    let num = 3;
    let count = move |x:i32| {
        //span 的所有权移动到闭包内了，但是从结果来看对于这种基础类型i32更多的是在闭包内复制了一个span不影响外部使用
        //这里仅仅是通过复制的效果，实现了移动span所有权，注意可变特性使用FnMut
        println!("closure span:{}",span);
        x+span
    };
    span += 2;
    let result = count(10);
    println!("num:{},span:{},result:{}",num,span,result);
}