macro_rules! tell_info {
    ($msg:expr $(, $($rest:tt)*)?) => {
        print!("{} ", console::style("[ 😊 ]").bold().black());
        println!($msg, $($($rest)*)?);
    }
}

macro_rules! tell_warn {
    ($msg:expr $(, $($rest:tt)*)?) => {
        print!("{} ", console::style("[ 🤭 ]").bold().yellow());
        println!($msg, $($($rest)*)?);
    }
}

macro_rules! tell_error {
    ($msg:expr $(, $($rest:tt)*)?) => {
        print!("{} ", console::style("[ 🙀 ]").bold().red().bright());
        println!($msg, $($($rest)*)?);
    }
}

macro_rules! tell_success {
    ($msg:expr $(, $($rest:tt)*)?) => {
        print!("{} ", console::style("[ 🥳 ]").bold().green());
        println!($msg, $($($rest)*)?);
    }
}

macro_rules! tell_failure {
    ($msg:expr $(, $($rest:tt)*)?) => {
        print!("{} ", console::style("[ 🤢 ]").bold().blue());
        println!($msg, $($($rest)*)?);
    }
}
