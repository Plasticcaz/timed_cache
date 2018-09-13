# Timed Cache

This library is an implementation of a cache that stores its contents by a key, as well as the duration of time that 
value will remain valid once it has been written to the cache.

If the value is missing when the user tries to retrieve a value from the `TimedCache`, or the value has outlived the 
specified storage `Duration` of the cache, the value will be regenerated using a specified function.

## Example:

```rust
    let cache = TimedCache::with_time_to_keep(Duration::from_seconds(60));
    cache.get(&"key".to_owned(), || some_mutexed_service.lock().unwrap().call());
```

For information, please see the tests and documentation.


## Contribution
I am more than willing to have help improving and extending this library. Please leave an issue and/or submit a pull 
request!