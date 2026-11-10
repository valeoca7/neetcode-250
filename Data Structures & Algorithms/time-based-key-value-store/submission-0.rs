// Time Based Key-Value Store

// Design a time-based key-value data structure that can store multiple values for the same key at different time stamps and retrieve the key's value at a certain timestamp.

// Implement the TimeMap class:

// TimeMap() Initializes the object of the data structure.
// void set(String key, String value, int timestamp) Stores the key key with the value value at the given time timestamp.
// String get(String key, int timestamp) Returns a value such that set was called previously, with timestamp_prev <= timestamp. If there are multiple such values, it returns the value associated with the largest timestamp_prev. If there are no values, it returns "".

// Example 1:

// Input:
// ["TimeMap", "set", ["alice", "happy", 1], "get", ["alice", 1], "get", ["alice", 2], "set", ["alice", "sad", 3], "get", ["alice", 3]]

// Output:
// [null, null, "happy", "happy", null, "sad"]

// Explanation:
// TimeMap timeMap = new TimeMap();
// timeMap.set("alice", "happy", 1);  // store the key "alice" and value "happy" along with timestamp = 1.
// timeMap.get("alice", 1);           // return "happy"
// timeMap.get("alice", 2);           // return "happy", there is no value stored for timestamp 2, thus we return the value at timestamp 1.
// timeMap.set("alice", "sad", 3);    // store the key "alice" and value "sad" along with timestamp = 3.
// timeMap.get("alice", 3);           // return "sad"

// Constraints:
// 1 <= key.length, value.length <= 100
// key and value only include lowercase English letters and digits.
// 0 <= timestamp <= 10^7
// All the timestamps of set are strictly increasing.
// At most 2 * 10^5 calls will be made to set and get.

// Recommended Time & Space Complexity
// You should aim for a solution with O(1) time for set(), O(logn) time for get(), and O(m * n) space, where n is the total number of values associated with a key, and m is the total number of keys.

//////////////////////////////////////////////////////////////////////////

use std::collections::HashMap;

struct TimeMap {
    entries: HashMap<String, Vec<(i32, String)>>,
}

impl TimeMap {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.entries
            .entry(key)
            .or_default()
            .push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        let Some(entries) = self.entries.get(&key) else {
            return String::new();
        };

        // Find the first entry with timestamp > target.
        let index = entries.partition_point(|&(ts, _)| ts <= timestamp);

        if index == 0 {
            String::new()
        } else {
            entries[index - 1].1.clone()
        }
    }
}
