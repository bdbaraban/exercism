// Defines a Clock struct.

use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    /// The current hour on the clock (24 hour format).
    hours: i32,
    /// The current minute on the clock.
    minutes: i32,
}

impl Clock {
    /// Instantiate a new Clock.
    pub fn new(hours: i32, minutes: i32) -> Self {
        const MINUTES_IN_DAY: i32 = 24 * 60;
        let min: i32 = ((minutes % MINUTES_IN_DAY) + MINUTES_IN_DAY) % MINUTES_IN_DAY;
        let hr: i32 = hours + min / 60;

        Clock {
            hours: ((hr % 24) + 24) % 24,
            minutes: min % 60,
        }
    }

    /// Add minutes to a Clock instance.
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

/// Display formatting for Clocks.
impl fmt::Display for Clock {
    fn fmt(&self, c: &mut fmt::Formatter) -> fmt::Result {
        write!(c, "{:02}:{:02}", self.hours, self.minutes)
    }
}
