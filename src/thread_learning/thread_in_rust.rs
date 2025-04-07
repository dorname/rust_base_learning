use std::thread;

fn f() {
    println!("hello world");
    let id = thread::current().id();
    println!("thread id: {:?}", id);
}

mod tests {
    use super::*;
    #[test]
    fn it_thread() {
        thread::spawn(f);
        thread::spawn(f);
        println!("main thread");
    }
}
