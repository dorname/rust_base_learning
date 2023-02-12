use std::thread;
use std::sync::mpsc;
use std::time::Duration;
fn thread_sender(){
    let (tx,rx) = mpsc::channel();
    thread::spawn(move||{
        tx.send("test").unwrap();
    });
    // println!("接受消息{}",rx.recv().unwrap());
    println!("try 接收信息{:?}",rx.try_recv());
    println!("try 接收信息{:?}",rx.try_recv());
    println!("try 接收信息{:?}",rx.try_recv());
}
fn thread_ownership(){
    let (tx,rx) = mpsc::channel();
    thread::spawn(move||{
        // let s:String = String::from("test");
        let i:i32 = 3;
        tx.send(i).unwrap();
        // println!("{}",s);
        println!("{}",i);
    });
    for re in rx{
        println!("test>>>>>>{}",re);
    }
    // println!("接收消息{}",rx.recv().unwrap());
}
fn multi_send_received(){
    let (sx,rx) = mpsc::channel();
    thread::spawn(move||{
        let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread")
        ];
        for val in vals{
            sx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx{
        println!("{}",received);
    }
}
fn multi_sender(){
    let (sx,rx) = mpsc::channel();
    let sx1 = sx.clone();
    thread::spawn(move||{
        sx.send("多发送者线程测试1>>>>").unwrap();
    });
    thread::spawn(move||{
        sx1.send("多发送者线程测试2>>>>").unwrap();
    });
    for received in rx{
        println!("{}",received);
    }
}
#[test]
fn test(){
    thread_sender();
    thread_ownership();
    multi_send_received();
    multi_sender();
}