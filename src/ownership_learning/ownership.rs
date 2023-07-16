fn owner_test(){
let x = 10;
let y = x;
println!("测试：{x},{y}");
}
fn owner_test_one(){
    let x = "helloworld";
    let y = x;
    println!("测试：{x},{y}");
    let a = 'c';
    let b = a;
    println!("测试:{a},{b}");

    let c = (1,2 as f64,"hello");
    let d = c;
    println!("测试:{:?},{:?}",c,d);

    let g = d.clone();

    println!("{:?}",g);

}
#[test]
fn test(){
    owner_test();
    owner_test_one();
}