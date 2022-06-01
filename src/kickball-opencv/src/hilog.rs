use log::Log;

pub struct HiLog;

impl Log for HiLog {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let level = match record.level() {
            log::Level::Trace => Level::Debug,
            log::Level::Debug => Level::Debug,
            log::Level::Info => Level::Info,
            log::Level::Warn => Level::Warn,
            log::Level::Error => Level::Error,
        };
        let msg = format!("[{}] {}", record.target(), record.args());
        log_print(level, msg)
    }

    fn flush(&self) {}
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
enum Level {
    Debug = 3,
    Info = 4,
    Warn = 5,
    Error = 6,
}

fn log_print(level: Level, mut msg: String) {
    msg.push('\u{0}');
    let ret = unsafe {
        HiLogPrint(
            0,
            level as i32,
            7890,
            "RUST napi\u{0}".as_ptr(),
            msg.as_bytes().as_ptr(),
        )
    };
    if ret < 0 {
        panic!("HiLog failed!")
    }
}

extern "C" {
    fn HiLogPrint(
        log_type: i32,
        log_level: i32,
        log_domain: std::os::raw::c_uint,
        log_tag: *const std::os::raw::c_char,
        fmt: *const std::os::raw::c_char,
        ...
    ) -> std::os::raw::c_int;
}
