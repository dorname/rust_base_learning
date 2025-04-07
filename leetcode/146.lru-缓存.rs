/*
 * @lc app=leetcode.cn id=146 lang=rust
 *
 * [146] LRU 缓存
 */
// @lc code=start
use std::collections::HashMap;
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
/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end

