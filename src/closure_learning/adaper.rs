#[test]
fn test(){
    let nums = vec![1,2,3,4,5,6];
    let total = nums.iter().sum::<i32>();
    println!("total:{total}");

    //收集0..100的偶数
    let nums_even:Vec<i32> = (0..100).filter(|n| {0== n%2}).collect();//collect是适配器
    println!("{:?}",nums_even);

    //求小于1000的能被3整除或5整除的所有整数之和
    let sum:i32 = (0..1000).filter(|n|{0==n%3||0==n%5}).sum::<i32>();
    println!("{}",sum);
}