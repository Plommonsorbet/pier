use prettytable::{cell, format, row, Table};
use serde::{Deserialize, Serialize};
use shell;
use snafu::{OptionExt, ResultExt};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use toml;

pub mod pier_error;
use pier_error::*;

pub type Result<T, E = pier_error::PierError> = ::std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    scripts: HashMap<String, Script>,
    #[serde(skip)]
    path: PathBuf,
}

impl Config {
    pub fn new(path: &Path) -> Config {
        Config {
            scripts: HashMap::new(),
            path: path.into(),
        }
    }

    pub fn new_config(path: &Path) -> Result<Config> {
        let config = Config::new(path);
        config.write()?;
        Ok(config)
    }

    pub fn write(&self) -> Result<()> {
        let mut file = File::create(&self.path).context(ConfigWrite { path: &self.path })?;
        let toml = toml::to_string_pretty(&self).context(TomlSerialize)?;
        file.write_all(toml.as_bytes())
            .context(ConfigWrite { path: &self.path })?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Config> {
        let mut config_string = String::new();
        File::open(&path)
            .context(ConfigRead { path: &path })?
            .read_to_string(&mut config_string)
            .context(ConfigRead { path: &path })?;

        let mut config: Config =
            toml::from_str(&config_string).context(TomlParse { path: &path })?;

        config.path = path.into();

        Ok(config)
    }

    pub fn from_input(selected_path: Option<&str>) -> Result<Config> {
        if let Some(path_str) = selected_path {
            let path = Path::new(path_str);
            if path.exists() {
                return Ok(Config::load(path)?);
            } else {
                return Ok(Config::new_config(path)?)
            }
        } else {
            // All possible default paths
            let paths: Vec<(&str, &str)> = vec![
                ("XDG_CONFIG_HOME", "pier/config"),
                ("HOME", ".config/pier/config"),
                ("HOME", ".pier"),
            ];

            for (env, relpath) in paths {
                // If environment variable exists
                if let Ok(e) = env::var(env) {
                    let path = format!("{}/{}", e, relpath);
                    // If path exists return with config file path
                    let path = Path::new(&path);
                    if path.exists() {
                        return Ok(Config::load(path)?)
                    };
                };
            }

            pier_err!(InnerPierError::NoConfigFile)
        };
    }

    pub fn fetch_script(&self, alias: &str) -> Result<&Script> {
        Ok(self
            .scripts
            .get(&alias.to_string())
            .context(AliasNotFound {
                alias: &alias.to_string(),
            })?)
    }
    pub fn add_script(&mut self, script: Script) -> Result<()> {
        self.scripts
            .entry(String::from(&script.alias))
            .or_insert(script);
        Ok(())
    }

    pub fn remove_script(&mut self, alias: &str) -> Result<()> {
        self.scripts
            .remove(&alias.to_string())
            .context(AliasNotFound {
                alias: &alias.to_string(),
            })?;
        Ok(())
    }

    pub fn list_scripts(&self) -> Result<()> {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        table.set_titles(row!["Alias", "Command"]);
        for (alias, script) in &self.scripts {
            table.add_row(row![alias, script.command]);
        }
        // Print the table to stdout
        table.printstd();
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    pub alias: String,
    pub command: String,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl Script {
    pub fn run(&self, arg: &str) -> Result<()> {
        println!("Starting script \"{}\"", &self.alias);
        println!("-------------------------");
        let default_shell = env::var("SHELL").context(NoDefaultShell)?;
        let command = shell::cmd!(&format!(
            "{} -c \"{} {}\"",
            default_shell, &self.command, &arg
        ));

        match command.stdout_utf8() {
            Ok(output) => {
                println!("{}", output);
                println!("-------------------------");
                println!("Script complete successfully.");
                Ok(())
            }
            Err(why) => pier_err!(InnerPierError::CommandFailed { why }),
        }
    }
}
