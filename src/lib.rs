use std::fs::File;
use std::io::{prelude::*};
use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};
use std::env;
use std::collections::HashMap;
use std::path::Path;
//use snafu::{ResultExt};
//mod error;
//use error::*;

use prettytable::{Table, row, cell, format};
use shell;

//use toml;
use serde::{Serialize, Deserialize};
//use snafu::{Snafu, ResultExt, ErrorCompat, IntoError};
use snafu;
//use std::io;
//use std::error;
use toml;


#[derive(Snafu, Debug)]
pub struct PierError(InnerPierError);

#[derive(Snafu, Debug)]
enum InnerPierError {
    #[snafu(display("error on {}: No scripts found for the alias.", "remove"))]
    AliasNotFound { action: &'static str },
    
    #[snafu(display("error: No config file found."))]
    ConfigFileNotFound { something: &'static str },
    
    //#[snafu(display("error: Unable to parse config from {}: {}", path, source))]
    //ConfigParseError {
    //    source: toml::de::Error,
    //    path: String
    //},

    #[snafu(display("error: Read error from {}: ", path))]
    ConfigReadError {
        source: std::io::Error,
        path: String
    },

    #[snafu(display("error: Write error from {}:", path))]
    ConfigWriteError {
        source: std::io::Error,
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
        return Err($type)?
    };
}

pub type Result<T, E = PierError> = ::std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    scripts: Option<HashMap<String, Script>>,
    #[serde(skip)]
    path: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    alias: String,
    command: String,
    description: Option<String>,
    reference: Option<String>,
    tags: Option<Vec<String>>,
}


impl Script {
    fn run(&self, arg: &str) -> Result<()> {
        println!("Starting script \"{}\"", &self.alias);
        println!("-------------------------");

        let default_shell = env::var("SHELL").expect("No default shell set!");
        let command = shell::cmd!(&format!("{} -c \"{} {}\"", default_shell, &self.command, &arg));
        match command.stdout_utf8() {
            Ok(output) => { 
                println!("{}", output);
                println!("-------------------------");
                println!("Script complete successfully.");
                Ok(())
            }
            Err(why) => {
                pier_err!(InnerPierError::CommandFailed{ why })
            }
        }
    }

}
impl Config {
    fn fetch_script(&self, alias: &str) -> Result<&Script> {
        match self.scripts {
            Some(ref scripts) => {
                match &scripts.get(&alias.to_string()) {
                    Some(script) => Ok(script),
                    None => pier_err!(InnerPierError::AliasNotFound { action: "fetch" })
                }
                
            }
            None => pier_err!(InnerPierError::NoScriptsFound) 
        }
    }
    fn add_script(&mut self, script: Script) -> Result<()> {
        match self.scripts {
            Some(ref mut scripts) => {
                scripts.entry(String::from(&script.alias)).or_insert(script);
                Ok(())}

            None => {
                let mut scripts = HashMap::new();
                scripts.insert(String::from(&script.alias), script);

                self.scripts = Some(scripts);
                Ok(())
            }
        }
        
    }

    fn remove_script(&mut self, alias: &str) -> Result<()> {
        match self.scripts {
            Some(ref mut scripts) => {
                match scripts.remove(alias) {
                    Some(_removed) => Ok(()),
                    None => pier_err!(InnerPierError::AliasNotFound { action: "remove" })
                    }
                }
            None => pier_err!(InnerPierError::NoScriptsFound) 
        }
    }

    fn list_scripts(&self) -> Result<()> {
        match self.scripts {
            Some(ref scripts) => {
                let mut table = Table::new();
                table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
                table.set_titles(row!["Alias", "Command"]);
                for (alias, script) in scripts {
                    table.add_row(row![alias, script.command]);
                }
                // Print the table to stdout
                table.printstd();
            }
            None => pier_err!(InnerPierError::NoScriptsFound)
            }
        Ok(())
    }

    fn write(&self) -> Result<()> {
        let mut file = File::create(&self.path).context(ConfigWriteError { path: &self.path.to_string() })?;
        let toml = toml::to_string(&self.scripts).expect("toml err");
        file.write_all(toml.as_bytes()).context(ConfigWriteError { path: &self.path.to_string() });

        Ok(())
    }

    pub fn from(path: &str) -> std::result::Result<Config, std::io::Error> {
        let mut config_string = String::new();
        File::open(path)?.read_to_string(&mut config_string)?;

        Ok(toml::from_str(&config_string).expect("Failed to parse"))

    }
}


pub fn get_config_file(select_path: Option<&str>) -> Result<String> {
    if let Some(path_str) = select_path {
        // return commandline argument passed or default environment variable found.
        return Ok(path_str.to_string())
    } else {
        // All possible default paths
        let paths: Vec<(&str, &str)> = vec![
            ("XDG_CONFIG_HOME", "pier/config"),
            ("XDG_CONFIG_HOME", "pier"),
            ("HOME", ".config/pier/config"),
            ("HOME", ".pier")
        ];

        for (env, relpath) in paths {
            // If environment variable exists
            if let Ok(e) = env::var(env) {
                let path = format!("{}/{}", e, relpath);
                // If path exists return with config file path
                if Path::new(&path).exists() {
                    return Ok(path)
                };
            };
        };

        pier_err!(InnerPierError::ConfigFileNotFound {something: "lol" });
    };
}

pub fn handle_subcommands(matches: &clap::ArgMatches, mut config: Config) -> Result<()> {
    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
            let command = sub_matches.value_of("INPUT").unwrap().to_string();
            let alias = sub_matches.value_of("alias").unwrap().to_string();
            let appendage = Script {
                alias: alias,
                command: command,
                description: None,
                reference: None,
                tags: None
            };

            config.add_script(appendage).expect("failed to add script (TMP)");
            config.write().context(ConfigWriteError { path: config.path.to_string() })?;
        }
        ("remove", Some(sub_matches)) => {
            let alias = sub_matches.value_of("INPUT").unwrap();
            config.remove_script(&alias).expect("failed to add script (TMP)");
            config.write()?;
            
        }
        ("run", Some(sub_matches)) => {
            let arg = "";
            let alias = sub_matches.value_of("INPUT").unwrap();
            let script = config.fetch_script(&alias).expect("failed to add script (TMP)");

            script.run(arg).expect("failed to add script (TMP)");
        }
        ("list", Some(_sub_matches)) => {
            config.list_scripts().expect("failed to add script (TMP)");
        },
        _ => {
            println!("woo");
            let arg = "";
            let alias = matches.value_of("INPUT").unwrap();
            let script = config.fetch_script(&alias).expect("failed to add script (TMP)");
            script.run(arg).expect("failed to add script (TMP)");

        }
    };
    Ok(())
}
