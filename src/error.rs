use snafu::{Snafu, ResultExt, ErrorCompat, IntoError};
use snafu;
use std::io;
use std::error;
use toml;

#[derive(Debug, Snafu)]
pub enum PierError {
    #[snafu(display("error on {}: No scripts found for the alias.", "remove"))]
    AliasNotFound { action: &'static str },
    
    #[snafu(display("error: No config file found."))]
    ConfigFileNotFound,
    
    #[snafu(display("error: Unable to parse config from {}: {}", path, source))]
    ConfigParseError {
        //#[snafu(source(from(toml::de::Error, Box::new)))]
        //source: ,
        source: toml::de::Error,
        path: String
    },

    #[snafu(display("error: Read error from {}: {}", path, source))]
    ConfigReadError {
        source: std::io::Error,
        path: String
    },

    #[snafu(display("error: Write error from {}: {}", path, source))]
    ConfigWriteError {
        source: io::Error,
        path: String
    },

    #[snafu(display("error: No scripts found."))]
    NoScriptsFound,
    
    #[snafu(display("error: Command failed: \n{:?}", why))]
    CommandFailed {
        why: shell::ShellError
    }
}


#[macro_export]
macro_rules! pier_err {
    ($type:expr) => {
        return Err($type)
    };
}
