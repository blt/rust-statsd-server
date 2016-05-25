/// piggy-back off a community library. It uses u64 internally--as do
/// hrdhistogram wrappers--so, uh, we'll just covert as needed. Adapting the
/// dependent library to be generic shouldn't be hard, I don't think.

use histogram;

pub struct Histogram {
    inner_hist: histogram::Histogram,
    scaling_factor: f64,
}

impl Histogram {
    pub fn new() -> Histogram {
        Histogram {
            inner_hist: histogram::Histogram::new().unwrap(),
            scaling_factor: 1000000.0,
        }
    }

    pub fn increment(&mut self, value: f64) -> Result<(), &'static str> {
        let u64_val = (value * self.scaling_factor).round() as u64;
        self.inner_hist.increment(u64_val)
    }

    pub fn percentile(&self, percent: f64) ->  Result<f64, &'static str> {
        match self.inner_hist.percentile(percent) {
            Result::Ok(val) => Result::Ok((val as f64) / self.scaling_factor),
            Result::Err(why) => Result::Err(why)
        }
    }
}
