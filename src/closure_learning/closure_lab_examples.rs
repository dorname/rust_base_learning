use std::vec;

#[test]
fn closure_test() {
    use std::mem;
    let color = String::from("green");

    let print = || println!("color: `{}`", color);
    print();

    let _color_moved = color;

    let mut count = 0;

    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };

    inc();
    // let _reborrow = &count;
    inc();

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let comsume_box = || {
        println!("comsume box: {:?}", movable);
        mem::drop(movable);
    };
    comsume_box();
    // comsume_box();

    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。

    // 在闭包的签名中删除 `move` 会导致闭包以不可变方式借用 `haystack`，因此之后
    // `haystack` 仍然可用，取消上面的注释也不会导致错误。
}

#[test]
fn input_closure() {
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    }
    use std::mem;
    let greeting = "hello";
    let mut farewell = "goodbye".to_string();
    let diary = || {
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. Good night!");
        mem::drop(farewell);
    };
    apply(diary);
}

#[test]
fn input_fn_closure() {
    let y = 4;
    fn call_me<F: Fn()>(f: F) {
        f();
    }
    fn function() {
        // error 即使参数换成了FnOnce，也会报错，因为普通函数无法调用转化成闭包的函数，无法捕获参数y
        // println!("I'm a function!{}", y);
        println!("I'm a function!");
    }
    let x = || println!("I'm a closure!");
    call_me(x);
    call_me(function);
}

#[test]
fn output_closure() {
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a: {}", text)
    }

    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a: {}", text)
    }
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    fn_plain();
    fn_mut();
    fn_once();
}

#[test]
fn closure_2() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));

    println!("Find 2 in vec2: {:?}", into_iter.any(|x| x == 2))
}
