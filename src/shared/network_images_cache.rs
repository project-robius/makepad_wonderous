use std::collections::{HashMap, VecDeque};

use makepad_widgets::LiveId;

pub struct NetworkImageCache {
    map: HashMap<LiveId, Vec<u8>>,
    order: VecDeque<LiveId>,
    capacity: usize,
}

impl NetworkImageCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn insert(&mut self, key: LiveId, value: Vec<u8>) {
        if self.map.contains_key(&key) {
            // Move the existing key to the front (most recently used)
            self.order.retain(|k| k != &key);
        } else if self.map.len() == self.capacity {
            if let Some(lru_key) = self.order.pop_back() {
                self.map.remove(&lru_key);
            }
        }
        self.map.insert(key.clone(), value);
        println!("Inserted key: {:?}", key);
        self.order.push_front(key);
    }

    pub fn get(&mut self, key: &LiveId) -> Option<&Vec<u8>> {
        if self.map.contains_key(key) {
            // Move the key to the front (most recently used)
            self.order.retain(|k| k != key);
            self.order.push_front(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }
}
