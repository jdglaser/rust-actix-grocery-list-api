use dotenv::dotenv;
use serde::Deserialize;

use lazy_static::lazy_static;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub database_url: String
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn get_config() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_config() {
        let config = get_config();
        println!("{:?}", config);
        assert!(config.database_url == "sqlite://:memory:")
    }
}