// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Earth;
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        const SECONDS_IN_YEAR: f64 = 31557600 as f64;
        d.seconds / SECONDS_IN_YEAR
    }
}

macro_rules! planet {
    ($name:ident, $coefficient:literal) => {
        pub struct $name;
        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                Earth::years_during(d) / $coefficient as f64
            }
        }
    };
}

planet!(Venus, 0.61519726);
planet!(Mercury, 0.2408467);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
