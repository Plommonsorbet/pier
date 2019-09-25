use pier::{Result, Config, Script};
use clap::load_yaml;
use clap::App;
use std::process;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    if let Err(err) = handle_subcommands(&matches) {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn handle_subcommands(matches: &clap::ArgMatches) -> Result<()> {
    let path = matches.value_of("config");
    let mut config = Config::from_input(path)?;
    match matches.subcommand() {
        ("add", Some(sub_matches)) => {
            let command = sub_matches.value_of("INPUT").unwrap().to_string();
            let alias = sub_matches.value_of("alias").unwrap().to_string();
            let appendage = Script {
                alias: alias,
                command: command,
                description: None,
                reference: None,
                tags: None,
            };

            config.add_script(appendage)?;
            config.write()?;
        }
        ("remove", Some(sub_matches)) => {
            let alias = sub_matches.value_of("INPUT").unwrap();
            //config.script_config
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
        }
        _ => {
            let arg = "";
            let alias = matches.value_of("INPUT").unwrap();
            let script = config.fetch_script(&alias)?;
            script.run(arg)?;
        }
    };
    Ok(())
}
