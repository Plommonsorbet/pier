use clap::load_yaml;
use clap::App;

use pier;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = &mut pier::load_config(&matches);

    match matches.value_of("INPUT") {
        Some(alias) => {
            // let arg = match sub_matches.value_of("arg") {
            //     Some(arg) => String::from(arg),
            //     None => String::from("")
            // };
            let arg = String::from("");

            match pier::fetch_script(alias, config) {
                Some(script) => pier::run_command(alias, &script.command, &arg),
                None => println!("Invalid alias, would you like to create a new script?"),
            }
        },
        None => pier::handle_subcommands(&matches, config).expect("No input or subcommands"),
    }    
}

