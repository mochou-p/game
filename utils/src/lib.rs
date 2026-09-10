// mochou-p/game/utils/src/lib.rs

#[macro_export]
macro_rules! style {
    ($color:expr, $tag:expr) => {
        concat!(
            "\x1b[10",
            stringify!($color),
            ";30;1m ",
            $tag,
            " @ ",
            file!(),
            ':',
            line!(),
            ':',
            column!(),
            " \x1b[0;3",
            stringify!($color),
            "m\n{}\x1b[0m\n"
        )
    };
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! log {
    ($color:expr, $tag:expr, $($arg:expr),+) => {
        println!(utils::style!($color, $tag), format!($($arg),+));
    };
}

#[macro_export]
macro_rules! elog {
    ($color:expr, $tag:expr, $($arg:expr),+) => {
        eprintln!(utils::style!($color, $tag), format!($($arg),+));
    };
}

////////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! debug {
    ($($arg:expr),+) => {
        utils::log!(7, "DEBUG", $($arg),+);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:expr),+) => {
        utils::log!(6, "INFO", $($arg),+);
    };
}

#[macro_export]
macro_rules! ok {
    ($($arg:expr),+) => {
        utils::log!(2, "OK", $($arg),+);
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:expr),+) => {
        utils::elog!(3, "WARNING", $($arg),+);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:expr),+) => {
        utils::elog!(1, "ERROR", $($arg),+);
    };
}

