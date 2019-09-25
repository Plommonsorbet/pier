use snafu;
use snafu::Snafu;
use std::path::PathBuf;
use toml;
#[derive(Snafu, Debug)]
pub struct PierError(InnerPierError);

#[derive(Snafu, Debug)]
#[snafu(visibility = "pub(crate)")]
pub enum InnerPierError {
    #[snafu(display("error: No script found by alias {}", alias))]
    AliasNotFound { alias: String },

    #[snafu(display("error: No default config file found. See help for more info."))]
    NoConfigFile,

    #[snafu(display(
        "error: Unable to serialize config: {}. Probably a bug in the code.",
        source
    ))]
    TomlSerialize { source: toml::ser::Error },

    #[snafu(display("error: Unable to parse toml config from file {}: {}", path.display(), source))]
    TomlParse {
        source: toml::de::Error,
        path: PathBuf,
    },
    #[snafu(display("error: Unable to read config from file {}: {} ", path.display(), source))]
    ConfigRead {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("error: Write error from {}: {}", path.display(), source))]
    ConfigWrite {
        source: std::io::Error,
        path: PathBuf,
    },

    #[snafu(display("error: No scripts found."))]
    NoScriptsFound,

    #[snafu(display("error: Command failed: \n{:?}", why))]
    CommandFailed { why: shell::ShellError },

    #[snafu(display("error: No default $SHELL: \n"))]
    NoDefaultShell { source: std::env::VarError },

    #[snafu(display("error: Expected value for {} \n", name))]
    CliValueNotFound { name: &'static str },
}

#[macro_export]
macro_rules! pier_err {
    ($type:expr) => {
        return Err($type)?;
    };
}
