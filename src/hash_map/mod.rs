use std::collections::HashMap;

#[test]
fn test() {
    #[derive(Clone, Debug)]
    struct LightLink {
        val: i32,
        prev: i32,
        next: i32,
    }

    impl LightLink {
        fn new(val: i32, prev: i32, next: i32) -> Self {
            Self { val, prev, next }
        }
        fn set_prev(&mut self, prev: i32) {
            self.prev = prev;
        }
        fn get_prev(&self) -> i32 {
            self.prev.clone()
        }
        fn get_prev_node(&self, map: &HashMap<i32, LightLink>) -> Option<LightLink> {
            map.get(&self.get_prev()).cloned()
        }
        fn set_next(&mut self, next: i32) {
            self.next = next;
        }
        fn get_next(&self) -> i32 {
            self.next.clone()
        }
        fn get_next_node(&self, map: &HashMap<i32, LightLink>) -> Option<LightLink> {
            map.get(&self.get_next()).cloned()
        }

        fn set_val(&mut self, val: i32) {
            self.val = val;
        }
        fn get_val(&self) -> i32 {
            self.val.clone()
        }
    }

    // @lc code=start
    struct LRUCache {
        caches: HashMap<i32, LightLink>,
        head: i32,
        tail: i32,
        capacity: usize,
    }

    /**
     * `&self` means the method takes an immutable reference.
     * If you need a mutable reference, change it to `&mut self` instead.
     */
    impl LRUCache {
        fn push_back(&mut self, key: i32) {
            //1、取出prev和next
            let node = self.caches.get(&key).cloned();
            match node {
                Some(mut link) => {
                    if key.eq(&self.tail) {
                        return;
                    }
                    let prev_node = link.get_prev_node(&self.caches);
                    let next_node = link.get_next_node(&self.caches);
                    if prev_node.is_some() {
                        let mut prev_link = prev_node.clone().unwrap();
                        prev_link.set_next(link.get_next());
                        self.caches.insert(link.get_prev(), prev_link);
                    }
                    if next_node.is_some() {
                        let mut next_link = next_node.unwrap();
                        next_link.set_prev(link.get_prev());
                        self.caches.insert(link.get_next(), next_link);
                        if prev_node.is_none() {
                            self.head = link.get_next();
                        }
                    }
                    let tail = self.caches.get(&self.tail).cloned();
                    if tail.is_some() {
                        let mut tail_link = tail.unwrap();
                        tail_link.set_next(key);
                        self.caches.insert(self.tail, tail_link);
                        link.set_prev(self.tail);
                    }
                    link.set_next(-1);
                    self.caches.insert(key, link);
                    self.tail = key;
                }
                _ => {}
            }
        }

        fn pop_front(&mut self) {
            let node = self.caches.get(&self.head).cloned();
            if let Some(link) = node {
                let next_node = link.get_next_node(&self.caches);
                if next_node.is_some() {
                    let mut next_link = next_node.unwrap();
                    next_link.set_prev(-1);
                    self.caches.insert(link.get_next(), next_link);
                }
                self.caches.remove(&self.head);
                self.head = link.get_next();
            }
        }

        fn new(capacity: i32) -> Self {
            Self {
                caches: HashMap::<i32, LightLink>::new(),
                head: -1,
                tail: -1,
                capacity: capacity as usize,
            }
        }
        fn get(&mut self, key: i32) -> i32 {
            let node = self.caches.get(&key).cloned();
            match node {
                Some(link) => {
                    self.push_back(key);
                    link.get_val()
                }
                _ => -1,
            }
        }

        fn put(&mut self, key: i32, value: i32) {
            let node = self.caches.get(&key).cloned();
            match node {
                Some(mut link) => {
                    link.set_val(value);
                    self.caches.insert(key, link);
                    self.push_back(key);
                }
                _ => {
                    if self.caches.is_empty() {
                        self.head = key;
                    }
                    if self.capacity < self.caches.len() + 1 {
                        self.pop_front();
                    }
                    if self.caches.is_empty() {
                        self.head = key;
                    }
                    let mut new_link = LightLink::new(value, self.tail, -1);
                    self.caches.insert(key, new_link);
                    let tail_node = self.caches.get(&self.tail).cloned();
                    if tail_node.is_some() {
                        let mut tail = tail_node.unwrap();
                        tail.set_next(key);
                        self.caches.insert(self.tail, tail);
                    }

                    self.tail = key;
                }
            }
        }
    }
    let mut lru = LRUCache::new(1);
    lru.get(1);
    lru.get(6);
    lru.get(8);
    lru.put(12, 1);
    println!("1、{:?}", lru.caches);
    lru.get(2);
    lru.put(15, 11);
    println!("2、{:?}", lru.caches);
    lru.put(5, 2);
    println!("3、{:?}", lru.caches);
    lru.put(1, 15);
    println!("4、{:?}", lru.caches);
    lru.put(4, 2);
    println!("5、{:?}", lru.caches);
    lru.get(5);
    lru.put(15, 15);
    // lru.put(1, 1);
    // lru.put(2, 2);
    // lru.put(3, 3);
    // lru.put(4, 4);
    // // 2 3 4
    // println!("1、{:?}", lru.caches);
    // lru.get(4);
    // // 2 3 4
    // println!("2、{:?}", lru.caches);
    // lru.get(3);
    // // 2 4 3
    // println!("3、{:?}", lru.caches);
    // lru.get(2);
    // // 4 3 2
    // println!("4、{:?}", lru.caches);
    // lru.get(1);
    // lru.put(5, 5);
    // println!("{:?}", lru.caches);
    // lru.get(1);
    // println!("{:?}", lru.caches);
    // lru.get(2);
    // println!("{:?}", lru.caches);
    // lru.get(3);
    // println!("{:?}", lru.caches);
    // lru.get(4);
    // println!("{:?}", lru.caches);
    // lru.get(5);
    // println!("{:?}", lru.caches);
    // lru.put(2, 1);
    // println!("{:?}", lru.caches);
    // lru.get(2);
    // lru.put(3, 2);
    // println!("{:?}", lru.caches);
    // lru.caches.insert(
    //     1,
    //     LightLink {
    //         val: 1,
    //         prev: -1,
    //         next: 2,
    //     },
    // );
    // lru.caches.insert(
    //     2,
    //     LightLink {
    //         val: 2,
    //         prev: 1,
    //         next: 3,
    //     },
    // );
    // lru.caches.insert(
    //     3,
    //     LightLink {
    //         val: 3,
    //         prev: 2,
    //         next: -1,
    //     },
    // );
    // lru.head = 1;
    // lru.tail = 3;
    // println!("{:?}", lru.caches);
    // lru.push_back(1);
    // println!("{:?}", lru.caches);
    // lru.push_back(3);
    // println!("{:?}", lru.caches);
    // lru.push_back(3);
    // println!("{:?}", lru.caches);
    // lru.pop_front();
    // println!("{:?}", lru.caches);
}
