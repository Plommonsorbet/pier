use std::fs::File;
use std::io::{prelude::*};
use std::io;
use std::error::Error;
use std::env;
use std::process;
use std::collections::HashMap;
use std::path::Path;
use clap::load_yaml;
use clap::App;

use simple_error::SimpleError;

#[macro_use] extern crate prettytable;
use prettytable::{Table, Row, Cell, format};

#[macro_use] extern crate shell;

use toml;
use serde::{Serialize, Deserialize};

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;

//#[derive(Debug)]
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    scripts: Option<HashMap<String, Script>>,
    #[serde(skip)]
    path: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Script {
    alias: String,
    command: String,
    description: Option<String>,
    reference: Option<String>,
    tags: Option<Vec<String>>,
}

macro_rules! err {
    ($message:expr) => {
        return Err(Box::new(SimpleError::new($message)))
    }
}


impl Script {
    fn run(&self, arg: &str) -> Result<()> {
        println!("Starting script \"{}\"", &self.alias);
        println!("-------------------------");

        let default_shell = env::var("SHELL").expect("No default shell set!");
        match cmd!(&format!("{} -c \"{} {}\"", default_shell, &self.command, &arg)).stdout_utf8() {
            Ok(output) => { 
                println!("{}", output);
                println!("-------------------------");
                println!("Script complete successfully.");
                Ok(())
            }
            Err(why) => {
                err!(format!("Command failed: {:?}", why))
            }
        }
    }

}
impl Config {
    fn new(config_path: &str) -> Config {
        Config {
            scripts: Some(HashMap::new()),
            path: config_path.to_string()
        }
    }
    fn fetch_script(self, alias: &str) -> Result<&Script> {
        match &self.scripts {
            Some(ref scripts) => {
                match &scripts.get(&alias.to_string()) {
                    Some(script) => Ok(script),
                    None => err!("Invalid alias, would you like to create a new script?")
                }
                
            }
            None => err!("No scripts found, would you like to create a new script?") 
        }
    }
    fn add_script(&mut self, script: Script) -> Result<()> {
        match self.scripts {
            Some(ref mut scripts) => {
                scripts.entry(String::from(&script.alias)).or_insert(script);
                Ok(())}
            None => Err("")?
        }
        
    }

    fn remove_script(&mut self, alias: &str) -> Result<()> {
        match self.scripts {
            Some(ref mut scripts) => {
                match scripts.remove(alias) {
                    Some(_removed) => Ok(()),
                    None => Err("")?
                    }
                }
            None => err!("Alias can't be found, nothing to delete.")
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
            }, None => err!("No scripts exist. Would you like to add a new script?") }
        Ok(())
    }

    fn write(&self) -> Result<()> {
        let mut file = File::create(&self.path)?;
        let toml = toml::to_string(&self.scripts)?;
        file.write_all(toml.as_bytes())?;

        Ok(())
    }

    fn load(config_path: &str) -> Result<Config> {
        let mut config_string = String::new();
       
        File::open(config_path)?.read_to_string(&mut config_string)?;

        Ok(toml::from_str(&config_string)?)

    }
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
   
    let config = Config::load(&cfg_file)?;
    println!("PAST LOAD CONFIG: {:?}", config);

    println!("config -> {:?}", config);

    //if let Some(path_str) =  {
    //};

    //let config = &mut load_config(&matches);

    match matches.value_of("INPUT") {
        Some(alias) => {
            let arg = "";
            let script = config.fetch_script(alias)?;
            
            script.run(arg)?;

        },
        None => handle_subcommands(&matches, config)?,
    };
    Ok(())
}

fn handle_subcommands(matches: &clap::ArgMatches, config: Config) -> Result<()> {
    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
            let appendage = Script {
                alias: sub_matches.value_of("alias").unwrap().to_string(),
                command: sub_matches.value_of("INPUT").unwrap().to_string(),
                description: None,
                reference: None,
                tags: None
            };

            config.add_script(appendage)?;
            config.write()?;
        }
        ("remove", Some(sub_matches)) => {
            let alias = sub_matches.value_of("alias").unwrap();
            config.remove_script(&alias)?;
            config.write()?;
            
        }
        ("run", Some(sub_matches)) => {
            let alias = sub_matches.value_of("alias").unwrap();
            let arg = "";
            let script = config.fetch_script(&alias)?;

            script.run(arg)?;
        }
        ("list", Some(_sub_matches)) => {
            config.list_scripts()?;
        }
        ("", None) => {
            err!("No scripts exist. Would you like to add a new script?")
        }
        _ => unreachable!()
    };
    Ok(())
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
