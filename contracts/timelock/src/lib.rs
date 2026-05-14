#![no_std]

pub const fn min_delay_seconds() -> u64 {
    86_400
}

pub fn is_ready(eta: u64, now: u64) -> bool {
    now >= eta
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_delay_is_one_day() {
        assert_eq!(min_delay_seconds(), 86_400);
    }

    #[test]
    fn ready_when_now_meets_eta() {
        assert!(is_ready(100, 100));
        assert!(is_ready(100, 200));
    }

    #[test]
    fn not_ready_when_before_eta() {
        assert!(!is_ready(100, 50));
    }
}
