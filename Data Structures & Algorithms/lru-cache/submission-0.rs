// LRU Cache
// Implement the Least Recently Used (LRU) cache class LRUCache. The class should support the following operations

// LRUCache(int capacity) Initialize the LRU cache of size capacity.
// int get(int key) Return the value corresponding to the key if the key exists, otherwise return -1.
// void put(int key, int value) Update the value of the key if the key exists. Otherwise, add the key-value pair to the cache. If the introduction of the new pair causes the cache to exceed its capacity, remove the least recently used key.
// A key is considered used if a get or a put operation is called on it.

// Ensure that get and put each run in O(1) average time complexity.

struct LRUCache {
    capacity: usize,
    nodes: Vec<Node>,
    entries: HashMap<i32, usize>,
    head: Option<usize>,
    tail: Option<usize>,
}

struct Node {
    key: i32, 
    value: i32, 
    prev: Option<usize>,
    next: Option<usize>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            capacity,
            nodes: Vec::with_capacity(capacity),
            entries: HashMap::with_capacity(capacity),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let Some(&index) = self.entries.get(&key) else {
            return -1;
        };
        let value = self.nodes[index].value;
        self.touch(index);
        value
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(&index) = self.entries.get(&key) {
            self.nodes[index].value = value;
            self.touch(index);
            return;
        }
        if self.nodes.len() < self.capacity {
            let index = self.nodes.len();
            self.nodes.push(Node {
                key,
                value,
                prev: None,
                next: None,
            });
            self.entries.insert(key, index);
            self.attach_to_tail(index);
            return;
        }
        // cache is full
        let victim = self
            .head
            .expect("a full cache must contain an LRU node");

        let old_key = self.nodes[victim].key;

        self.detach(victim);
        self.entries.remove(&old_key);

        self.nodes[victim].key = key;
        self.nodes[victim].value = value;
        self.nodes[victim].prev = None;
        self.nodes[victim].next = None;
        
        self.entries.insert(key, victim);
        self.attach_to_tail(victim);
    }

    // marks an existing node as the most recently used
    fn touch(&mut self, index: usize) {
        if self.tail == Some(index) {
            return;
        }
        self.detach(index);
        self.attach_to_tail(index);
    }

    // removes a node from the recently list
    fn detach(&mut self, index: usize) {
        let prev = self.nodes[index].prev;
        let next = self.nodes[index].next;

        match prev {
            Some(prev_index) => {
                self.nodes[prev_index].next = next;
            }
            None => {
                self.head = next;
            }
        }

        match next {
            Some(next_index) => {
                self.nodes[next_index].prev = prev;
            }
            None => {
                self.tail = prev;
            }
        }

        self.nodes[index].prev = None;
        self.nodes[index].next = None;
    }

    // appends a detached node to the MRU end of the recency list
    fn attach_to_tail(&mut self, index: usize) {
        self.nodes[index].prev = self.tail;
        self.nodes[index].next = None;

        match self.tail {
            Some(old_tail) => {
                self.nodes[old_tail].next = Some(index);
            }
            None => {
                self.head = Some(index);
            }
        }
        self.tail = Some(index)
    }
}
