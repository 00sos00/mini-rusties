#![allow(dead_code)]
/* use std::panic::Location;
use colored::*; */

/* pub struct Logger;

impl Logger {
    fn get_time() -> String {
        let current_time = chrono::Utc::now().to_string();
        let current_time_string = current_time.as_str()[0..19].replace(" ", "_");

        current_time_string
    }

    #[track_caller]
    pub fn success(message: &str) {
        //println!("[{}]", Self::get_time());

        println!(
            "[{log_level}] {message}",
            log_level = Paint::green("SUCCESS"),
            message = Paint::green(message),
        );
    }

    #[track_caller]
    pub fn debug(message: &str) {
        //println!("[{}]", Self::get_time());

        println!(
            "[{log_level}] {message} [{file}:{line}:{column}]",
            log_level = Paint::blue("DEBUG"),
            message = Paint::blue(message),
            file = Location::caller().file(),
            line = Location::caller().line(),
            column = Location::caller().column(),
        );
    }

    #[track_caller]
    pub fn warn(message: &str) {
        //println!("[{}]", Self::get_time());

        println!(
            "[{log_level}] {message} [{file}:{line}:{column}]",
            log_level = Paint::yellow("WARN"),
            message = Paint::yellow(message),
            file = Location::caller().file(),
            line = Location::caller().line(),
            column = Location::caller().column(),
        );
    }

    #[track_caller]
    pub fn error(message: &str) {
        //println!("[{}]", Self::get_time());

        let message = format!(
            "[{log_level}] {message}",
            log_level = Paint::red("ERR"),
            message = Paint::red(message),
        );

        panic!("{}", message);
    }
}
 */