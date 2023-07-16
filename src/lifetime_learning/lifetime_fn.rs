pub fn longest<'l>(x: &'l str, y: &'l str) -> &'l str {
    if x.len() > y.len() {
        println!("{}", 1);
        x
    } else {
        println!("{}", 2);
        y
    }
}
pub fn longest_one<'l>(x: &'l str, y: &str) -> &'l str {
    x
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn static_lifetime_test() {
    use std::fmt::Debug;
    let i = 5;
    //声明一个生命周期为'static的常量i
    // let i: &'static i32 = &5;
    fn print_it<T: Debug + 'static>(input: T) {
        println!("1 'static value passed in is: {:?}", input);
    }

    fn print_it1(input: impl Debug + 'static) {
        println!("2 'static value passed in is: {:?}", input);
    }

    fn print_it2<T: Debug + 'static>(input: &T) {
        println!("3 'static value passed in is: {:?}", input);
    }

    fn lifetime_test<'a>() -> &'a i32{
        let x:&'a i32 = &10;
        
        print!("{x}");
        x
    }
    fn lifetime_test_one<'a>(y:&'a mut String) -> &String{
        let x:String = String::from("hello");
        print!("{x}");
        *y = x;
        y
    }

    /**
     *以下代码会引起生命周期的编译错误
     */
    // print_it(&i);
    // print_it1(&i);

    print_it(i);
    print_it1(i);
    print_it2(&i);
    lifetime_test();
}
#[test]
fn test() {
    let s1 = String::from("test1");
    let s2 = String::from("test2");
    let s3 = String::from("test one two");
    longest(&s1, &s2);
    longest_one(&s1, &s2);
    println!("{}", first_word(&s3));
    static_lifetime_test();
}
