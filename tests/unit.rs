use pier::{Config, Script, get_config_file, handle_subcommands, Result};

fn toml_config_success() -> Result<Config> {
    Config::from("tests/assets/success.config.toml")
}

fn toml_config_fail() -> Result<Config> {
    Config::from("tests/assets/fail.config.toml")
}

#[test]
fn parse_from_toml_success () {
    assert!(toml_config_success().is_ok());
}

#[test]
fn parse_from_toml_fail () {
    assert!(toml_config_fail().is_err());
    println!("{}", toml_config_fail().unwrap_err());
}

