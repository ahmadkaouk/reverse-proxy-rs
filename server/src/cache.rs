use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;
use std::time::Instant;

/// Cache operations
pub trait Cache<K: Hash + Eq, V> {
    /// Get the value associeted to `key` in the cache, return None if `key` is not if present
    fn get(&self, key: &K) -> Option<V>;
    /// Search for entry in the cache. If the entry is found, check if expired, if true remove it
    /// and return None, else return the value
    fn fetch(&self, key: &K) -> Option<V>;
    /// Insert a new entry (key, value) in the cache
    fn insert(&self, key: K, value: V);
    /// Remove the Entry associated to `key` in the cache
    fn remove(&self, key: &K);
    /// Remove expired caches based on the cache ejiction policy
    fn flush(&self);
    /// Remove all the entries from the cache
    fn clear(&self);
}

/// Entry with timestamp
pub trait TimedEntry {
    fn set_timestamp(&mut self, timestamp: Instant);
    fn timestamp(&self) -> Instant;
}

/// `TTLCacheEntry` is a cache entry that consists of a timestamp of insertion
/// a value
#[derive(Debug, Clone)]
pub struct TTLCacheEntry<V> {
    timestamp: Instant,
    value: V,
}

impl<V: Clone> TTLCacheEntry<V> {
    pub fn new(value: V) -> TTLCacheEntry<V> {
        TTLCacheEntry {
            timestamp: Instant::now(),
            value,
        }
    }
    pub fn value(&self) -> V {
        self.value.clone()
    }
}

impl<V> TimedEntry for TTLCacheEntry<V> {
    fn set_timestamp(&mut self, timestamp: Instant) {
        self.timestamp = timestamp;
    }

    fn timestamp(&self) -> Instant {
        self.timestamp
    }
}

/// Cache using Time To Live as eviction policy
///
/// Values are timestamped at insertion, and expired
/// entries are removed from the cache
#[derive(Debug, Default)]
pub struct TTLCache<K, V> {
    /// Cache Store encapsulated in a reader-writer lock
    store: RwLock<HashMap<K, V>>,
    /// Time To Live of an entry (in seconds)
    ttl: u64,
}

impl<K: Hash + Eq, V> TTLCache<K, V> {
    /// Creates a new `TTLcache` with the given ttl
    pub fn new(ttl: u64) -> TTLCache<K, V> {
        TTLCache {
            store: RwLock::new(HashMap::new()),
            ttl,
        }
    }
}

impl<K, V> Cache<K, V> for TTLCache<K, V>
where
    K: Hash + Eq,
    V: Clone + TimedEntry,
{
    fn get(&self, key: &K) -> Option<V> {
        self.store.read().unwrap().get(key).cloned()
    }

    fn insert(&self, key: K, value: V) {
        let mut v = value;
        v.set_timestamp(Instant::now());
        self.store.write().unwrap().insert(key, v);
    }

    fn remove(&self, key: &K) {
        self.store.write().unwrap().remove(key);
    }

    fn flush(&self) {
        self.store
            .write()
            .unwrap()
            .retain(|_, v| v.timestamp().elapsed().as_secs() < self.ttl)
    }

    fn clear(&self) {
        self.store.write().unwrap().clear()
    }

    fn fetch(&self, key: &K) -> Option<V> {
        self.get(key).and_then(|v| {
            if v.timestamp().elapsed().as_secs() < self.ttl {
                Some(v)
            } else {
                self.remove(key);
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instantion() {
        let cache = TTLCache::<String, TTLCacheEntry<String>>::new(30);
    }
}
