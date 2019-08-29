use std::fs::File;
use std::io::{prelude::*, Error};
use std::env;
use std::process;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use shellexpand;

use clap::load_yaml;
use clap::App;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format};

#[macro_use] extern crate shell;

use toml;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    scripts: Option<HashMap<String, Script>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Script {
    alias: String,
    command: String,
    description: Option<String>,
    reference: Option<String>,
    tags: Option<Vec<String>>,
}


fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let cfg_file = get_config_file(matches.value_of("config"));
    
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
//fn fetch_script<'a>(alias: &str, config: &'a Config) -> Option<&'a Script> {
//    return match &config.scripts {
//        Some(scripts) => {
//            scripts.get(&alias.to_string())
//        },
//        None => None
//    }
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
//
//fn write_config(config_path: &str, config: &Config) -> Result<(),Error> {
//    let mut file = File::create(config_path)?;
//    
//    let toml = toml::to_string(config).unwrap();
//    file.write_all(toml.as_bytes())
//        .expect("Could not write to file!");
//    
//    Ok(())
//}
//
fn load_config(config_path: &str) -> Config {
    let mut config_string = String::new();
    
    match File::open(config_path) {
        Ok(mut file) => {
            file.read_to_string(&mut config_string)
                .expect("Failed to read config file contents");
        },
        Err(_error) => {
            panic!("Config file {} not found", &config_path);
            process::exit(1);
        },
    };

    toml::from_str(&config_string).unwrap()
}

fn get_config_file(select_path: Option<&str>) -> String {
    if let Some(path_str) = select_path {
        // If commandline argument passed or environment variable found.
        return path_str.to_string()
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
                    return path
                };
            };
        };

        println!("No config files found.");
        process::exit(1);
    }
}
