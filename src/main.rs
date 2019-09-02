use std::fs::File;
use std::io::{prelude::*};
use std::io;
use std::error::Error;
use std::env;
use std::process;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use shellexpand;
use std::fmt;
use clap::load_yaml;
use clap::App;

use simple_error::SimpleError;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format};

#[macro_use] extern crate shell;

use toml;
use serde::{Serialize, Deserialize};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    scripts: Option<HashMap<String, Script>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Script {
    alias: String,
    command: Option<String>,
    description: Option<String>,
    reference: Option<String>,
    tags: Option<Vec<String>>,
}

macro_rules! err {
    ($message:expr) => {
        return Err(Box::new(SimpleError::new($message)))
    }
}

//fn fetch_script<'a>(alias: &str, config: &'a Config) -> Option<&'a Script> {
//    return match &config.scripts {
//        Some(scripts) => {
//            scripts.get(&alias.to_string())
//        },
//        None => None
//    }
//}

impl Script {
    fn add(self, config: &mut Config) -> Result<()> {
        match config.scripts {
            Some(ref mut scripts) => {
                scripts.entry(String::from(&self.alias)).or_insert(self);
                Ok(())}
            None => Err("")?
        }
        
    }

    fn remove(self, alias: &str, config: &mut Config) -> Result<()> {
        match config.scripts {
            Some(ref mut scripts) => {
                match scripts.remove(&self.alias) {
                    Some(_removed) => Ok(()),
                    None => Err("")?
                    }
                }
            None => err!("No command found by that alias")
        }
        
    }
}
impl Config {
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    if let Err(err) = try_main(matches) {
        eprintln!("{}", err);
        process::exit(1);
    }


}

fn try_main(matches: clap::ArgMatches) -> Result<()> {
    let cfg_file = get_config_file(matches.value_of("config"))?;
    println!("cfg_file -> {}", cfg_file);
   
    let config = load_config(&cfg_file)
        .map_err(|error| format!("Load config file {}: {}", cfg_file, error))?;

    println!("PAST LOAD CONFIG");

    //if let Err(err) = {
    //    format!("Load config file {}: {}", cfg_file, error)
    //    eprintln!("Failed to load config file {}:", err);
    //    //.map_err(|error| format!("Load config file {}: {}", cfg_file, error))?;
    //}
    //let config = match load_config(&cfg_file) {
    //    Ok(cfg) => Ok(cfg),
    //    Err(err) => {
    //        Err(format!("Load config file {}: {}", cfg_file, err))
    //    }
    //}?;
    println!("config -> {:?}", config);

    Ok(())
    //if let Some(path_str) =  {
    //};

    //let config = &mut load_config(&matches);

    //match matches.value_of("INPUT") {
    //    Some(alias) => {
    //        // let arg = match sub_matches.value_of("arg") {
    //        //     Some(arg) => String::from(arg),
    //        //     None => String::from("")
    //        // };
    //        let arg = String::from("");

    //        match fetch_script(alias, config) {
    //            Some(script) => run_command(alias, &script.command, &arg),
    //            None => println!("Invalid alias, would you like to create a new script?"),
    //        }
    //    },
    //    None => handle_subcommands(&matches, config).expect("No input or subcommands"),
    //}    
}


//fn handle_subcommands(matches: &clap::ArgMatches, config: & mut Config) -> Result<(),Error> {
//    match matches.subcommand() {
//        ("add", Some(sub_matches)) => {
//            let command = sub_matches.value_of("INPUT").unwrap();
//            let alias = sub_matches.value_of("alias").unwrap();
//           
//            let appendage = Script {
//                alias: alias.to_string(),
//                command: command.to_string(), 
//                description: None,
//                reference: None,
//                tags: None
//            };
//
//            match &config.scripts {
//                Some(_scripts) => {
//                    config.scripts.as_mut().unwrap()
//                        .entry(alias.to_string()).or_insert(appendage);
//                    write_config(&matches, &config)
//                        .expect("Failed to save config to file");
//                },
//                None => {
//                    let mut scripts = HashMap::new();
//                    scripts.insert(alias.to_string(), appendage);
//                    write_config(
//                        &matches, 
//                        &Config {
//                            scripts: Some(scripts)
//                        })
//                        .expect("Failed to save config to file");
//                }
//            }
//
//            println!("+ {} / alias {}", command, alias);
//        },
//        ("remove", Some(sub_matches)) => {
//            let alias = sub_matches.value_of("INPUT").unwrap();
//            let script: Script;
//
//            match &config.scripts {
//                Some(scripts) => {
//                    if scripts.contains_key(&alias.to_string()) {
//                        script = config.scripts.as_mut().unwrap()
//                            .remove(&alias.to_string())
//                            .expect("Failed to remove script");
//                        write_config(&matches, &config)
//                            .expect("Failed to save config to file");
//                    } else {
//                        println!("Invalid alias");
//                        process::exit(1);
//                    } 
//                },
//                None => {
//                    println!("Invalid alias");
//                    process::exit(1);
//                }
//            }
//
//            println!("- {:?} / alias {}", script, alias);
//        },
//        ("run", Some(sub_matches)) => {
//            let alias = sub_matches.value_of("INPUT").unwrap();
//            let arg = match sub_matches.value_of("arg") {
//                Some(arg) => String::from(arg),
//                None => String::from("")
//            };
//
//            match fetch_script(alias, config) {
//                Some(script) => run_command(alias, &script.command, &arg),
//                None => println!("Invalid alias, would you like to create a new script?"),
//            }
//        },
//        ("list", Some(_sub_matches)) => {
//            match &config.scripts {
//                Some(scripts) => {
//                    let mut table = Table::new();
//                    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
//                    table.set_titles(row!["Alias", "Command"]);
//                    for (alias, script) in scripts {
//                        table.add_row(row![alias, script.command]);
//                    }
//                    // Print the table to stdout
//                    table.printstd();
//                },
//                None => println!("No scripts exist. Would you like to add a new script?")
//            }
//        },
//        ("", None) => println!("No subcommand was used"),
//        _          => unreachable!(),
//    }
//
//    Ok(())
//}
//


//fn run_command(alias: &str, command: &str, arg: &str) {
//    println!("Starting script \"{}\"", alias);
//    println!("-------------------------");
//    
//    let default_shell = env::var("SHELL").expect("No default shell set!");
//    let output = cmd!(&format!("{} -c \"{} {}\"", default_shell, command, arg)).stdout_utf8().unwrap();
//    println!("{}", output);
//
//    println!("-------------------------");
//    println!("Script complete");
//}


fn write_config(config_path: &str, config: &Config) -> Result<()> {
    let mut file = File::create(config_path)?;
    
    let toml = toml::to_string(config).unwrap();
    file.write_all(toml.as_bytes())
        .expect("Could not write to file!"); 
    Ok(())
}
//
fn load_config(config_path: &str) -> Result<Config> {
    let mut config_string = String::new();
   
    File::open(config_path)?.read_to_string(&mut config_string)?;

    Ok(toml::from_str(&config_string)?)
}

fn get_config_file(select_path: Option<&str>) -> Result<String> {
    if let Some(path_str) = select_path {
        // If commandline argument passed or environment variable found.
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

        Err(io::Error::new(io::ErrorKind::NotFound, "No Config file found!"))?
    }
}
