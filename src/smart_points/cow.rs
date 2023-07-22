use std::{borrow::Cow, time::Instant};
fn filter_space(src: &str) -> String {
    //使用with_capacity函数预置一个src长度大小内存空间，空间具有伸展性，随着字符串的溢出而伸展
    let mut target = String::with_capacity(src.len());
    for c in src.chars() {
        if ' ' != c {
            target.push(c);
        }
    }
    target
}

//使用枚举智能指针cow
fn filter_space_cow<'a>(src: &'a str) -> Cow<'a, str> {
    let mut target = String::with_capacity(src.len());
    if src.contains(' ') {
        for c in src.chars() {
            if ' ' != c {
                target.push(c);
            }
        }
        return Cow::Owned(target);
    }
    Cow::Borrowed(src)
}

//性能测试
fn compare() {
   
    let input = "Hello, Rust! This is a test string with spaces.";
    let iterations = 1000000;
  

    // 使用 Cow 智能指针函数性能测试
    let start = Instant::now();
    for _ in 0..iterations {
        filter_space_cow(input);
    }
    let duration = start.elapsed();
    println!(
        "Cow function: {} microseconds per iteration",
        duration.as_micros() as f64 / iterations as f64
    );

     // 原始函数性能测试
    let start = Instant::now();
    for _ in 0..iterations {
        filter_space(input);
    }
    let duration = start.elapsed();
    println!(
        "Original function: {} microseconds per iteration",
        duration.as_micros() as f64 / iterations as f64
    );
}
#[test]
fn test() {
    // println!("{}", filter_space("hello yyy"));
    // println!("{}", filter_space_cow("hello lpz"));

    compare();
}
