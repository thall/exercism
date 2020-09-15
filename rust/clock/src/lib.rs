const MINUTES_OF_A_DAY: i32 = 1440;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug)]
pub struct Clock {
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minute: convert_to_minutes(hours, minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            minute: mod_(self.minute + minutes, MINUTES_OF_A_DAY),
        }
    }
}

impl std::string::ToString for Clock {
    fn to_string(&self) -> String {
        const HOURS_OF_A_DAY: i32 = 24;

        let hours = mod_(self.minute / MINUTES_PER_HOUR, HOURS_OF_A_DAY);
        let minutes = mod_(self.minute, MINUTES_PER_HOUR);
        format!("{:02}:{:02}", hours, minutes)
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minute == other.minute
    }
}

fn convert_to_minutes(hours: i32, minutes: i32) -> i32 {
    let m = hours * MINUTES_PER_HOUR +  minutes;
    mod_(m, MINUTES_OF_A_DAY)
}

fn mod_(x: i32, m: i32) -> i32 {
    let r = x % m;
    if r < 0 {
        r + m
    } else {
        r
    }
}
