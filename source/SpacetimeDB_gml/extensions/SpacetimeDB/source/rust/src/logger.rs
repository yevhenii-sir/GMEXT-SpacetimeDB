//! Simple global logger for the `log` crate.
//!
//! Controlled via `stdb_set_log_level(level)` from GML:
//!   0 = Off, 1 = Error, 2 = Warn, 3 = Info, 4 = Debug, 5 = Trace
//!
//! Logs are written to stderr (visible in GameMaker's debug console on
//! most platforms). The logger is initialized lazily on first call to
//! `stdb_set_log_level` or when any `log::trace!`/`log::warn!` etc. fires.

use log::{LevelFilter, Metadata, Record};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

/// Global log level stored as an atomic u8.
/// Default is 2 (Warn) so that `log::warn!` and `log::error!` are visible.
static LOG_LEVEL: AtomicU8 = AtomicU8::new(2);

/// Whether the logger has been initialized.
static LOGGER_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Convert our 0-5 level to `log::LevelFilter`.
fn level_from_u8(level: u8) -> LevelFilter {
    match level {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }
}

/// The concrete logger implementation.
struct GmlLogger;

impl log::Log for GmlLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let current = level_from_u8(LOG_LEVEL.load(Ordering::Relaxed));
        metadata.level() <= current
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            eprintln!("[spacetimedb][{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {
        // stderr is unbuffered by default, no-op
    }
}

static LOGGER: GmlLogger = GmlLogger;

/// Initialize the global logger. Safe to call multiple times (idempotent).
fn ensure_logger_initialized() {
    if LOGGER_INITIALIZED
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        // First time: install the logger
        let level = level_from_u8(LOG_LEVEL.load(Ordering::Relaxed));
        if log::set_logger(&LOGGER).is_ok() {
            log::set_max_level(level);
        } else {
            // Another logger was already set — that's fine
            LOGGER_INITIALIZED.store(false, Ordering::SeqCst);
        }
    }
}

/// Set the log level from FFI. Called as `stdb_set_log_level(level)`.
///
/// Levels: 0=Off, 1=Error, 2=Warn (default), 3=Info, 4=Debug, 5=Trace
pub fn set_log_level(level: u8) {
    ensure_logger_initialized();
    let clamped = level.min(5);
    LOG_LEVEL.store(clamped, Ordering::Relaxed);
    log::set_max_level(level_from_u8(clamped));
}