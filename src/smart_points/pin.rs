use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::*;

// 定义一个简单的异步任务
struct SimpleFuture {
    counter: u32,
}

impl Future for SimpleFuture {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.counter > 0 {
            // 模拟异步等待
            match Pin::new(&mut self).counter {
                1 => {
                    // 第一次异步等待
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
                2 => {
                    // 第二次异步等待
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
                _ => {
                    self.counter -= 1;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        } else {
            Poll::Ready("Async operation completed!".to_string())
        }
    }
}

#[tokio::test]
async fn test() {
     // 创建一个 Pin<Box<Future<Output = String>>> 对象
    let future: Pin<Box<dyn Future<Output = String>>> = Box::pin(SimpleFuture { counter: 3 });

    // 将异步任务固定在内存中，防止在异步执行过程中被移动
    let pinned_future = future;
  
    // 异步等待任务的完成
    let result = pinned_future.await;
    println!("{}", result);
    // 输出结果："Async operation completed!"
}
