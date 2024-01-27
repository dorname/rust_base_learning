
#[test]
fn into_iter_test(){
    let vec_t = vec![1,2,3,4];
    vec_t.iter().for_each(|x|{
        println!("测试{:?}",x);
    });
    let nums = vec![1, 2, 3, 4, 5];
    // 使用 try_for_each 处理元素
    let result = nums.iter().try_for_each(|&num| {
        if num <= 3 {
            println!("处理: {}", num);
            Ok(())
        } else {
            Err(format!("数字 {} 太大了", num))
        }
    });

    // 检查结果
    match result {
        Ok(_) => println!("所有数字都成功处理"),
        Err(e) => println!("处理出错: {}", e),
    }
}
#[test]
fn iter_filter() {
    let nums = vec![1,2,3,4,5,6,7,8,8,8,9];
    let iter_test:Vec<&i32> = nums.iter().filter(|&&x|{
        return x>=8;
    }).collect();
    println!("{:?}",iter_test);
}
#[test]
fn iter_find(){
    let nums = vec![1,8,3,4,5,6,7,8,8,8,9];
    let iter_test = nums.iter().find(|&&x|{
        return x>=7;
    });
    println!("{:?}",iter_test.unwrap());
}