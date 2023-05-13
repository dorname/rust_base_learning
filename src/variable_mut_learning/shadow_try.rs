fn variablie(){
    let x = 5;
    let x = x + 2;
    {
        let x = x * x;
        println!("作用域内x = {}",x);
    }
    println!("变量x = {}",x);
}
fn variablie_mut(){
    let mut x = 5;
    x = x+1;
    {
        // let x = x * x;
        x = x * x;
        println!("在作用域内可变变量x = {}",x);
    }
    // println!("可变变量x = {}",x);
    println!("可变变量x = {x}");
}
#[test]
fn test(){
    variablie();
    variablie_mut();
}