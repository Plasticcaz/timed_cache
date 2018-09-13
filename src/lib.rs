//!
//! # Example
//!
//! The following example is obviously overly simplistic, but gives an idea of how one might use
//! this library. In a real-world application, one might have the generator function call an
//! over-the-network service to get some session key.
//!
//! ```
//!    extern crate timed_cache;
//!
//!    use timed_cache::TimedCache;
//!
//!    use std::time::Duration;
//!    use std::sync::Mutex;
//!
//!    struct TestService(usize);
//!
//!    impl TestService {
//!        fn next(&mut self) -> usize {
//!            let n = self.0;
//!            self.0 += 1;
//!            n
//!        }
//!    }
//!
//!    let time_to_keep = Duration::from_millis(1);
//!    let mut cache = TimedCache::<String, usize>::with_time_to_keep(time_to_keep);
//!
//!    let service = Mutex::new(TestService(0));
//!
//!    let generate_value = || service.lock().unwrap().next();
//!
//!    (0..1000).for_each(|_| {
//!        // this generator method will
//!        let value = cache.get(&"value".to_owned(), generate_value);
//!        println!("{}", value);
//!    });
//! ```
//!
mod timed_data;

use std::collections::HashMap;
use std::hash::Hash;
use std::time::Duration;
use timed_data::TimedData;

///
/// A collection which stores a value for a set amount of time.
///
/// Each stored value will keep track of when it was stored, and once the value has been considered
/// valid for the specified length of time, it will be re-generated using the generator function
/// provided when attempting to retrieve a value using `TimedCache::get`.
///
pub struct TimedCache<Key: Hash + Eq + Clone, Value> {
    ///
    /// The amount of time a value will be considered 'valid'.
    ///
    time_to_keep: Duration,
    ///
    /// The place this storage will be held.
    ///
    store: HashMap<Key, TimedData<Value>>,
}

impl<Key: Hash + Eq + Clone, Value> TimedCache<Key, Value> {
    ///
    /// Creates a `TimedCache` with the specified `Duration` as the length of time the values will
    /// be considered 'valid' after initial storage.
    ///
    pub fn with_time_to_keep(time_to_keep: Duration) -> TimedCache<Key, Value> {
        TimedCache {
            time_to_keep,
            store: HashMap::new(),
        }
    }

    ///
    /// Retrieves a reference to the value stored in the cache for the `key` if it exists and
    /// is still considered valid, otherwise calls `generate_value` to generate the value to
    /// store in the cache and returns a reference to that value.
    ///
    pub fn get(&mut self, key: &Key, generate_value: impl Fn() -> Value) -> &Value {
        // NOTE(zac):
        // I tried to write this using `Option`s, but ran into borrow checker problems.
        // So this is what I ended up with.
        // TODO(zac): See if, in the future, you can convert this to use `Option`s without
        // the borrow checker throwing a fit.
        if self.present_and_valid(key) {
            &self.store[key].item
        } else {
            self.insert_and_retrieve(key, generate_value)
        }
    }

    fn present_and_valid(&self, key: &Key) -> bool {
        self.store
            .get(key)
            .filter(|timed_data| timed_data.still_valid(self.time_to_keep))
            .is_some()
    }

    fn insert_and_retrieve(&mut self, key: &Key, generate_value: impl Fn() -> Value) -> &Value {
        let value = generate_value();
        // Throw away any old value, it's not important for this use case.
        let _ = self.store.insert(key.clone(), TimedData::new(value));
        &self.store[key].item
    }
}

#[cfg(test)]
mod tests {
    use super::TimedCache;
    use std::sync::Mutex;
    use std::thread::sleep;
    use std::time::Duration;

    const KEY: &str = "test";

    struct TestService(usize);

    impl TestService {
        fn next(&mut self) -> usize {
            let n = self.0;
            self.0 += 1;
            n
        }
    }

    #[test]
    fn should_create_test_from_duration() {
        TimedCache::<String, usize>::with_time_to_keep(Duration::from_millis(3));
    }

    #[test]
    fn should_contain_same_value_for_key_within_duration() {
        let service = Mutex::new(TestService(0));
        let generate_value = || service.lock().unwrap().next();

        let mut cache = TimedCache::<String, usize>::with_time_to_keep(Duration::from_secs(10));

        let a = *cache.get(&KEY.to_owned(), generate_value);
        let b = *cache.get(&KEY.to_owned(), generate_value);

        assert_eq!(a, 0);
        assert_eq!(b, 0);
    }

    #[test]
    fn should_contain_different_value_for_key_after_duration() {
        let service = Mutex::new(TestService(0));
        let generate_value = || service.lock().unwrap().next();

        let mut cache = TimedCache::<String, usize>::with_time_to_keep(Duration::from_millis(5));

        let a = *cache.get(&KEY.to_owned(), generate_value);
        sleep(Duration::from_millis(5));
        let b = *cache.get(&KEY.to_owned(), generate_value);

        assert_ne!(a, b);
        assert_eq!(a, 0);
        assert_eq!(b, 1);
    }

}
