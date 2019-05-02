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
        let mut hr: i32 = hours;
        let mut min: i32 = minutes;

        if min >= 60 {
            hr += min / 60;
            min %= 60;
        } else if min % 60 == 0 {
            hr -= min.abs() / 60;
            min = 0;
        } else if min.is_negative() {
            hr -= min.abs() / 60 + 1;
            min = 60 - min.abs() % 60;
        }

        if hr >= 24 {
            hr %= 24;
        } else if hr % 24 == 0 {
            hr = 0;
        } else if hr < 0 {
            hr = 24 - hr.abs() % 24;
        }

        Clock {
            hours: hr,
            minutes: min,
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
