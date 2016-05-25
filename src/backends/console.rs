use super::super::backend::Backend;
use super::super::buckets::Buckets;
use time;

#[derive(Debug)]
pub struct Console;


impl Console {
    /// Create a Console formatter that prints to stdout
    ///
    /// # Examples
    ///
    /// ```
    /// let cons = Console::new();
    /// ```
    pub fn new() -> Console {
        Console
    }
}

/// Print a single stats line.
fn fmt_line(key: &str, value: &f64) {
    println!("    {}: {}", key, value)
}


impl Backend for Console {
    fn flush(&mut self, buckets: &Buckets) {
        let now = time::get_time();
        println!("Flushing metrics: {}", time::at(now).rfc822().to_string());

        println!("  bad_messages: {}", buckets.bad_messages());
        println!("  total_messages: {}", buckets.total_messages());

        println!("  counters:");
        for (key, value) in buckets.counters() {
            fmt_line(&key, &value);
        }

        println!("  gauges:");
        for (key, value) in buckets.gauges() {
            fmt_line(&key, &value);
        }


        println!("  histograms:");
        for (key, value) in buckets.histograms() {
            // println!("    {}: {} 50th", key, value.percentile(50.0));
            // println!("    {}: {} 90th", key, value.percentile(90.0));
            // println!("    {}: {} 99th", key, value.percentile(99.0));
            // println!("    {}: {} 99.9th", key, value.percentile(99.9));
        }

        println!("  timers:");
        for (key, values) in buckets.timers() {
            println!("    {}: {:?}", key, values);
        }

        println!("  timer_data:");
        for (key, values) in buckets.timer_data() {
            println!("    {}: {:?}", key, values);
        }
    }
}
