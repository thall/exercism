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

    fn years(d: &Duration, factor: f64) -> f64 {
        Earth::years_during(d) / factor
    }
}

pub struct Mercury;

pub struct Venus;

pub struct Earth;

pub struct Mars;

pub struct Jupiter;

pub struct Saturn;

pub struct Uranus;

pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        Self::years(d, 0.2408467 as f64)
    }
}

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        Self::years(d, 0.61519726 as f64)
    }
}

impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        const SECONDS_IN_YEAR: f64 = 31557600 as f64;
        d.seconds / SECONDS_IN_YEAR
    }
}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        Self::years(d, 1.8808158 as f64)
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        Self::years(d, 11.862615 as f64)
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        Self::years(d, 29.447498 as f64)
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        Self::years(d, 84.016846 as f64)
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        Self::years(d, 164.79132 as f64)
    }
}
