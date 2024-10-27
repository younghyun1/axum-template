use chrono::{DateTime, Utc};
use tracing::info;

pub struct Stopwatch {
    original_start: tokio::time::Instant,
    start: tokio::time::Instant,
}

impl Stopwatch {
    pub fn new(message: Option<&str>) -> Self {
        match message {
            Some(msg) => {
                println!(
                    "{}  INFO {}: {}",
                    chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true),
                    env!("CARGO_PKG_NAME"),
                    msg
                );
            }
            None => (),
        }

        let now = tokio::time::Instant::now();

        return Self {
            original_start: now,
            start: now,
        };
    }

    pub fn click(&mut self, message: &str) {
        info!("{}: {:?}", message, self.start.elapsed());
        self.start = tokio::time::Instant::now();
    }

    pub fn total(&self, message: &str) {
        info!("{}: {:?}", message, self.original_start.elapsed())
    }
}
