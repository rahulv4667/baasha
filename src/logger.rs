use std::process;

#[allow(dead_code,non_snake_case)]
#[derive(Debug)]
pub enum LogLevel {
    ERROR = 1,
    WARNING = 2,
    CRASH = 3
}

#[allow(non_snake_case)]
pub fn log_message(level: LogLevel, col: usize, line: usize, s: String) {
    match level {
        LogLevel::ERROR   => { println!("[ERROR] [Line: {}, Col: {} ] {}", line, col, s); }
        LogLevel::WARNING => { println!("[WARN]  [Line: {}, Col: {} ] {}", line, col, s); }
        LogLevel::CRASH   => { println!("[CRASH] [Line: {}, Col: {} ] {}", line, col, s); process::exit(1); }
    }
}