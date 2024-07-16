use std::{collections::{HashMap, VecDeque}, rc::Rc};

use makepad_widgets::LiveId;
// TODO: reduce the default capacity once all visual bugs are fixed.
const DEFAULT_CAPACITY_BYTES: usize = 50 * 1024 * 1024; // 50 MB

pub struct NetworkImageCache {
    map: HashMap<LiveId, Rc<[u8]>>,
    order: VecDeque<LiveId>,
    capacity_bytes: usize,
    current_size_bytes: usize,
}

impl NetworkImageCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            order: VecDeque::new(),
            capacity_bytes: DEFAULT_CAPACITY_BYTES,
            current_size_bytes: 0,
        }
    }

    pub fn insert(&mut self, key: LiveId, value: &[u8]) {
        let rc_value = Rc::from(value);
        let value_size = value.len();
        if self.map.contains_key(&key) {
            // Move the existing key to the front (most recently used)
            self.order.retain(|k| k != &key);
            // Adjust the current size by removing the old size
            self.current_size_bytes -= self.map[&key].len();
        } else if self.current_size_bytes + value_size > self.capacity_bytes {
            // Remove the least recently used items until there's enough space
            while self.current_size_bytes + value_size > self.capacity_bytes {
                if let Some(lru_key) = self.order.pop_back() {
                    if let Some(lru_value) = self.map.remove(&lru_key) {
                        self.current_size_bytes -= lru_value.len();
                    }
                }
            }
        }
        // Insert the new item
        self.map.insert(key.clone(), rc_value);
        self.order.push_front(key);
        self.current_size_bytes += value_size;
    }

    pub fn get(&mut self, key: &LiveId) -> Option<Rc<[u8]>> {
        if self.map.contains_key(key) {
            // Move the key to the front (most recently used)
            self.order.retain(|k| k != key);
            self.order.push_front(key.clone());
            self.map.get(key).cloned()
        } else {
            None
        }
    }
}
