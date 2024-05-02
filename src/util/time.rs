use std::time::{Duration, SystemTime};

pub(crate) fn current_time_with_seconds(seconds: u64) -> SystemTime {
    SystemTime::now() + Duration::from_secs(seconds)
}

pub(crate) fn current_time_with_milliseconds(milliseconds: u64) -> SystemTime {
    SystemTime::now() + Duration::from_millis(milliseconds)
}

pub(crate) fn is_expired(expiry_time: SystemTime) -> bool {
    SystemTime::now().duration_since(expiry_time).is_ok()
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use super::is_expired;

    use super::current_time_with_milliseconds;

    use super::current_time_with_seconds;

    #[test]
    fn test_current_time_with_seconds() {
        let now = SystemTime::now();
        let t = current_time_with_seconds(1);

        assert!(t > now);
    }

    #[test]
    fn test_current_time_with_milliseconds() {
        let now = SystemTime::now();
        let t = current_time_with_milliseconds(1);

        assert!(t > now);
    }

    #[test]
    fn test_is_expired() {
        let t = current_time_with_milliseconds(100);

        assert!(!is_expired(t))
    }
}
