use core::{fmt};
use spin::Mutex;

static LOG_LEVEL_NAMES: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];
static GLOBAL_LOG_LEVEL: Mutex<Logger> = Mutex::new(Logger { level: Level::Off });

struct Logger {
    level: Level,
}

#[repr(usize)]
#[derive(Copy, Eq, Debug, Hash)]
pub enum Level {
    Off = 0,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl fmt::Display for Level {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.pad(self.as_str())
    }
}

impl Clone for Level {
    #[inline]
    fn clone(&self) -> Level {
        *self
    }
}

impl PartialEq for Level {
    #[inline]
    fn eq(&self, other: &Level) -> bool {
        *self as usize == *other as usize
    }
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        LOG_LEVEL_NAMES[*self as usize]
    }
}

pub fn print_log(level: Level, args: fmt::Arguments) {
    if level as usize > GLOBAL_LOG_LEVEL.lock().level as usize {
        return ();
    }
    let color_ansi = match level {
        Level::Info => 34,
        Level::Warn => 93,
        Level::Error => 31,
        Level::Debug => 32,
        Level::Trace => 90,
        _ => 0
    };
    print!("\x1b[{}m{} - {}\x1b[0m", color_ansi, level, args)
}

pub fn init_log_level(level: Level) {
    GLOBAL_LOG_LEVEL.lock().level = level;
}

#[macro_export]
#[allow_internal_unstable(format_args_nl)]
macro_rules! info {
    ($($arg:tt)*) => ({
        $crate::log::print_log($crate::log::Level::Info, format_args_nl!($($arg)*));
    })
}

#[macro_export]
#[allow_internal_unstable(format_args_nl)]
macro_rules! warn {
    ($($arg:tt)*) => ({
        $crate::log::print_log($crate::log::Level::Warn, format_args_nl!($($arg)*));
    })
}

#[macro_export]
#[allow_internal_unstable(format_args_nl)]
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::log::print_log($crate::log::Level::Error, format_args_nl!($($arg)*));
    })
}

#[macro_export]
#[allow_internal_unstable(format_args_nl)]
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::log::print_log($crate::log::Level::Debug, format_args_nl!($($arg)*));
    })
}

#[macro_export]
#[allow_internal_unstable(format_args_nl)]
macro_rules! trace {
    ($($arg:tt)*) => ({
        $crate::log::print_log($crate::log::Level::Trace, format_args_nl!($($arg)*));
    })
}

#[macro_export]
#[allow_internal_unstable(format_args_nl)]
macro_rules! log {
    ($lvl:expr, $($arg:tt)+) => ({
        $crate::log::print_log($lvl, format_args_nl!($($arg)*));
    })
}

