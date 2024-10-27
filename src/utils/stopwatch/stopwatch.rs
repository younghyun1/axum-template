use tracing::info;

pub struct Stopwatch {
    original_start: tokio::time::Instant,
    start: tokio::time::Instant,
}

impl Stopwatch {
    pub fn new(message: Option<&str>) -> Self {
        if let Some(msg) = message {
            println!(
                "{}  INFO {}: {}",
                chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Micros, true),
                env!("CARGO_PKG_NAME"),
                msg
            );
        }

        let now = tokio::time::Instant::now();

        Self {
            original_start: now,
            start: now,
        }
    }

    pub fn click(&mut self, message: &str) {
        info!("{}: {:?}", message, self.start.elapsed());
        self.start = tokio::time::Instant::now();
    }

    pub fn total(&self, message: &str) {
        info!("{}: {:?}", message, self.original_start.elapsed())
    }
}
