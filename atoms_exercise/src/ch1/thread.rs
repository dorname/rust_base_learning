use std::thread;

mod tests {
    use core::num;

    use super::*;

    #[test]
    fn thread_id_test() {
        thread::spawn(f);
        thread::spawn(f);
        println!("Hello from the main thread.");
    }

    #[test]
    fn thread_test2() {
        let t1 = thread::spawn(f);
        let t2 = thread::spawn(f);
        println!("Hello from the main thread.");
        t1.join().unwrap();
        t2.join().unwrap();
    }

    #[test]
    fn thread_test3() {
        let nums = vec![1, 2, 3];
        for n in nums {
            thread::spawn(move || {
                println!("{n}");
            })
            .join()
            .unwrap();
        }
    }

    #[test]
    fn thread_test4() {
        let numbers = Vec::from_iter(0..=1000);
        let t = thread::spawn(move || {
            let len = numbers.len();
            let sum = numbers.iter().sum::<usize>();
            sum / len
        });
        let average = t.join().unwrap();
        println!("Average: {average}");
    }

    #[test]
    fn thread_test5() {
        let mut nums = vec![1, 2, 3];
        thread::scope(|s| {
            s.spawn(|| {
                nums.push(4);
            });
            // s.spawn(|| {
            //     nums.push(5); //error
            // });
        });
    }

    #[test]
    fn thread_test6() {
        static X: [i32; 3] = [1, 2, 3];

        thread::spawn(|| dbg!(&X));
        thread::spawn(|| dbg!(&X));
    }

    #[test]
    fn thread_test7() {
        let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

        thread::spawn(move || dbg!(x));
        thread::spawn(move || dbg!(x));
    }

    #[test]
    fn thread_test8() {
        use std::rc::Rc;

        let a = Rc::new([1, 2, 3]);
        let b = a.clone();
        println!("a: {:?}, b: {:?}", a.as_ptr(), b.as_ptr());
        assert_eq!(a.as_ptr(), b.as_ptr()); // Same allocation!
    }

    #[test]
    fn thread_test9() {
        use std::sync::Arc;

        let a = Arc::new([1, 2, 3]);
        let b = a.clone();

        thread::spawn(move || dbg!(a));
        thread::spawn(move || dbg!(b));
    }
    #[test]
    fn thread_test10() {
        let a = 1;
        let mut b = 2;
        f1(&a, &mut b);
    }

    #[test]
    fn cell_test() {
        use std::cell::Cell;
        fn f(a: &Cell<i32>, b: &Cell<i32>) {
            let before = a.get();
            b.set(b.get() + 1);
            let after = a.get();
            if before != after {
                x(); // never happens
            }
        }
        fn x() {
            dbg!()
        }
        let a = Cell::new(1);
        let b = Cell::new(2);
        f(&a, &b);
    }

    #[test]
    fn parrel_test() {
        use std::sync::Mutex;
        use std::time::Duration;
        let n = Mutex::new(0);
        thread::scope(|s| {
            for _ in 0..10 {
                s.spawn(|| {
                    let mut guard = n.lock().unwrap();
                    for _ in 0..100 {
                        *guard += 1;
                    }
                    println!("{guard}");
                    drop(guard);
                    thread::sleep(Duration::from_secs(1)); // New!
                });
            }
        });
        assert_eq!(n.into_inner().unwrap(), 1000);
    }
}

fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}

fn f1(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        x(); // never happens
    }
}
fn x() {
    dbg!()
}
