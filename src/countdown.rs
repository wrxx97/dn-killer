use std::time::{Duration, Instant};

pub struct Countdown {
    pub deadline: Instant,
    duration: Duration,
    reminder_sent: bool,
}

impl Countdown {
    pub fn new(started_at: Instant, duration: Duration) -> Self {
        Self {
            deadline: started_at + duration,
            duration,
            reminder_sent: false,
        }
    }

    pub fn remaining(&self, now: Instant) -> Duration {
        self.deadline.saturating_duration_since(now)
    }

    pub fn progress(&self, now: Instant) -> f32 {
        self.remaining(now).as_secs_f32() / self.duration.as_secs_f32()
    }

    pub fn take_reminder(&mut self, now: Instant, lead: Duration) -> Option<u64> {
        let left = self.remaining(now);
        if self.reminder_sent || left.is_zero() || left > lead {
            return None;
        }
        self.reminder_sent = true;
        Some(ceil_seconds(left))
    }
}

pub fn ceil_seconds(duration: Duration) -> u64 {
    duration.as_secs() + u64::from(duration.subsec_nanos() != 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    const LENGTH: Duration = Duration::from_secs(15);
    const LEAD: Duration = Duration::from_secs(10);

    #[test]
    fn reminds_once_after_crossing_threshold() {
        let start = Instant::now();
        let mut countdown = Countdown::new(start, LENGTH);
        assert_eq!(
            countdown.take_reminder(start + Duration::from_secs(4), LEAD),
            None
        );
        assert_eq!(
            countdown.take_reminder(start + Duration::from_millis(6100), LEAD),
            Some(9)
        );
        assert_eq!(
            countdown.take_reminder(start + Duration::from_secs(7), LEAD),
            None
        );
    }

    #[test]
    fn restarting_countdown_allows_a_new_reminder() {
        let start = Instant::now();
        let mut countdown = Countdown::new(start, LENGTH);
        assert_eq!(
            countdown.take_reminder(start + Duration::from_secs(5), LEAD),
            Some(10)
        );
        let restart = start + Duration::from_secs(6);
        countdown = Countdown::new(restart, LENGTH);
        assert_eq!(
            countdown.take_reminder(restart + Duration::from_secs(5), LEAD),
            Some(10)
        );
    }

    #[test]
    fn no_voice_reminder_after_expiry() {
        let start = Instant::now();
        let mut countdown = Countdown::new(start, LENGTH);
        assert_eq!(countdown.take_reminder(start + LENGTH, LEAD), None);
    }
}
