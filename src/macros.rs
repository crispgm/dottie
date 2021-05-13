#[macro_export]
macro_rules! show_info {
    () => (println!());
    ($($arg:tt)*) => ({
        (println!($($arg)*));
    });
}

#[macro_export]
macro_rules! show_warn {
    ($($arg:tt)*) => ({
        extern crate colored;
        use colored::*;
        let prefix = "WARN".yellow();
        (println!("{} {}",prefix,format!($($arg)*)));
    })
}

#[macro_export]
macro_rules! show_error {
    ($($arg:tt)*) => ({
        extern crate colored;
        use colored::*;
        let prefix = "ERROR".red();
        (eprintln!("{} {}",prefix,format!($($arg)*)));
    })
}
