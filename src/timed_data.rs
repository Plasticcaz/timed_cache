use std::time::Duration;
use std::time::Instant;

///
/// A container to host a value-instant pair.
///
pub(crate) struct TimedData<T> {
    pub(crate) item: T,
    pub(crate) time_stored: Instant,
}

impl<T> TimedData<T> {
    pub(crate) fn new(item: T) -> TimedData<T> {
        TimedData {
            item,
            time_stored: Instant::now(),
        }
    }

    pub(crate) fn still_valid(&self, time_to_live: Duration) -> bool {
        // NOTE(zac):
        // A token is still valid if it has not been alive for longer than the
        // specified time_to_live.
        let time_lived_thus_far = Instant::now() - self.time_stored;
        time_to_live > time_lived_thus_far
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn should_be_considered_valid_if_within_duration() {
        let time_to_live = Duration::from_secs(10);
        let timed_data = TimedData::new(5);
        assert!(timed_data.still_valid(time_to_live));
    }


    #[test]
    fn should_not_be_considered_valid_if_after_duration() {
        let time_to_live = Duration::from_millis(5);
        let timed_data = TimedData::new(5);
        sleep(time_to_live);
        assert!(!timed_data.still_valid(time_to_live));
    }
}
