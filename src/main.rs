use pier::{Result, Config, get_config_file, handle_subcommands, PierError};
use clap::load_yaml;
use clap::App;
use std::process;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    if let Err(err) = try_main(matches) {
        eprintln!("{:?}", err);
        process::exit(1);
    }


}

fn try_main(matches: clap::ArgMatches) -> std::result::Result<(), PierError> {
    let cfg_file = get_config_file(matches.value_of("config"))?;
   
    let config = Config::from(&cfg_file).expect("failed to add script (TMP)");

    handle_subcommands(&matches, config).expect("failed to add script (TMP)");
    //match matches.value_of("INPUT") {
    //    Some(alias) => {
    //        let arg = "";
    //        let script = config.fetch_script(alias)?;
    //        
    //        script.run(arg)?;

    //    },
    //    None => handle_subcommands(&matches, config)?,
    //};
    Ok(())
}

