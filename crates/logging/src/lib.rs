use log::{Metadata, Record};
use std::sync::atomic::{AtomicU64, Ordering};
use uuid::Uuid;

static CORRELATION_ID: AtomicU64 = AtomicU64::new(0);

pub struct AesLogger;

impl log::Log for AesLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let ts = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
            let cid = Uuid::new_v4().to_string()[..8].to_string();
            println!("[{}] [{}] [{}] [{}] {}", ts, record.level(), record.target(), cid, record.args());
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    log::set_boxed_logger(Box::new(AesLogger))
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .ok();
}

pub fn set_correlation_id() {
    CORRELATION_ID.fetch_add(1, Ordering::SeqCst);
}
