fn closure_fn() {
    let x = 1;
    let sum = |y| x + y;
    println!("{}", sum(2));
}
fn closure_fn_one() {
    use std::thread;
    use std::time::Duration;
    // 开始健身，好累，我得发出声音：muuuu...
    fn muuuuu(intensity: u32) -> u32 {
        println!("muuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    }
    fn workout(intensity: u32, random_number: u32) {
        let action = muuuuu;
        if intensity < 25 {
            println!("今天活力满满, 先做 {} 个俯卧撑!", action(intensity));
            println!(
                "旁边有妹子在看，俯卧撑太low, 再来 {} 组卧推!",
                action(intensity)
            );
        } else if random_number == 3 {
            println!("昨天练过度了，今天还是休息下吧！");
        } else {
            println!(
                "昨天练过度了，今天干干有氧, 跑步 {} 分钟!",
                action(intensity)
            );
        }
    }
    // 强度
    let intensity = 25;
    // 随机值用来决定某个选择
    let random_number = 7;

    // 开始健身
    workout(intensity, random_number);
}

//闭包学习
//结构体中的闭包
fn closure_fn_two() {
    
    let mut s = |x:u32|x;

    //为结构体Test_Fn 实现Fn(u32) -> u32特征Trait
    struct Test_Fn;
    
    impl Test_Fn {
        fn test_fn(x:u32) -> u32 {
            x
        }
    }    

    // impl Fn<(i32,), i32> for Test_Fn {
    //     extern "rust-call" fn call(&self, (x,): (i32,)) -> i32 {
    //         x + 1
    //     }
    // }

    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        query: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(query: T) -> Cacher<T> {
            Cacher { query, value: None }
        }

        // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
        fn value(&mut self, arg: u32) -> u32 {
            // println!("{}", self);
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    let mut a= Cacher::new(s);
    let mut b = Cacher::new(Test_Fn::test_fn);
    let mut c = Cacher::new(|g|g);

    let v1 = c.value(1);
    let v2 = c.value(2);
    println!("{}=?{}",(c.query)(2),v2);
    println!("{}=?{}",b.value(2),b.value(1));
    assert_eq!(v1, 1);
}
#[test]
fn test() {
    closure_fn();
    closure_fn_one();
    closure_fn_two();
}
