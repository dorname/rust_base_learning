fn iter_test(){
    let nums = vec![1,2,3,4,5,6,7,8];
    for num in nums.iter(){
        println!("num:{}",num+1);
    }
    println!("nums:{:?}",nums);
}
fn iter_mut_test(){
    let mut nums = vec![1,2,3,4,5,6,7,8];
    for num in nums.iter_mut(){
        *num = *num+2;
        println!("num:{}",num);
    }
    println!("nums:{:?}",nums);
}
fn into_iter_test(){
    let nums = vec![1,2,3,4,5,6,7,8];
    for num in nums.into_iter(){
        println!("num:{}",num);
    }
    // 等价于
    // for num in nums.into_iter(){
    //     println!("num:{}",num);
    // }
    // println!("nums:{:?}",nums); //borrow of moved value: `nums` ,value borrowed here after move
}
#[test]
fn test(){
    // iter_test();
    // iter_mut_test();
    into_iter_test();
}