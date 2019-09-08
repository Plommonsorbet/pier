use snafu::{Snafu};

#[derive(Debug, Snafu)]
pub enum PierError {
    #[snafu(display("error on {}: No scripts found for the alias.", "remove"))]
    AliasNotFound { action: &'static str },
    
    #[snafu(display("error: No config file found."))]
    ConfigFileNotFound,

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
        return Err(Box::new($type))
    };
}
