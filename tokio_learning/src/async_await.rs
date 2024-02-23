use std::{time::Duration};

async fn fetch_data() -> u32 {
    // 模拟异步操作，比如从网络获取数据
    42 // 假设这是我们获取的数据
}
async fn process_data() {
    let data = fetch_data().await;
    println!("Data: {}", data);
}

async fn task1(){
    println!("Task 1 start");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Task 1 end");
}

async fn task2(){
    println!("Task 2 start");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("Task 2 end");
}
async fn task3(){
    println!("Task 3 start");
    tokio::time::sleep(Duration::from_secs(1)).await;
    println!("Task 3 end");
}

#[tokio::test]
async fn test(){
    // process_data().await;
    // println!("thisData: {}",45);
        let t1 = task1();
        let t2 = task2();
        let t3 = task3();
        tokio::join!(t2,t1,t3);
}