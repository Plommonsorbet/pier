use std::fs::File;
use std::io::{prelude::*};
use std::error::Error;
use std::env;
use std::collections::HashMap;
use std::path::Path;
use snafu::{ResultExt};
mod error;
use error::*;

use prettytable::{Table, row, cell, format};
use shell;

use toml;
use serde::{Serialize, Deserialize};

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
                pier_err!(PierError::CommandFailed{ why })
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
                    None => pier_err!(PierError::AliasNotFound { action: "fetch" })
                }
                
            }
            None => pier_err!(PierError::NoScriptsFound) 
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
                    None => pier_err!(PierError::AliasNotFound { action: "remove" })
                    }
                }
            None => pier_err!(PierError::NoScriptsFound) 
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
            None => pier_err!(PierError::NoScriptsFound)
            }
        Ok(())
    }

    fn write(&self) -> Result<()> {
        let mut file = File::create(&self.path)?;
        let toml = toml::to_string(&self.scripts)?;
        file.write_all(toml.as_bytes())?;

        Ok(())
    }

    pub fn from(config_path: &str) -> Result<Config> {
        let mut config_string = String::new();
        File::open(config_path).context(PierError::ConfigReadError { path: config_path.to_string() })?.read_to_string(&mut config_string)?;

        Ok(toml::from_str(&config_string).context(PierError::ConfigParseError { path: config_path.to_string() } )?)

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

        pier_err!(PierError::ConfigFileNotFound);
    }
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

            config.add_script(appendage)?;
            config.write()?;
        }
        ("remove", Some(sub_matches)) => {
            let alias = sub_matches.value_of("INPUT").unwrap();
            config.remove_script(&alias)?;
            config.write()?;
            
        }
        ("run", Some(sub_matches)) => {
            let arg = "";
            let alias = sub_matches.value_of("INPUT").unwrap();
            let script = config.fetch_script(&alias)?;

            script.run(arg)?;
        }
        ("list", Some(_sub_matches)) => {
            config.list_scripts()?;
        },
        _ => {
            println!("woo");
            let arg = "";
            let alias = matches.value_of("INPUT").unwrap();
            let script = config.fetch_script(&alias)?;
            script.run(arg)?;

        }
    };
    Ok(())
}
