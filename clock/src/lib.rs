use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let mut total_mins = hours * 60 + minutes;
        if total_mins < 0 {
            total_mins += (1 - total_mins / (24 * 60)) * 24 * 60
        }
        Clock { hours : (total_mins / 60) % 24, minutes : total_mins % 60 }
    }
    pub fn add_minutes(&self, minutes : i32) -> Clock {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
