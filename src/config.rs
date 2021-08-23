use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub database_type: String,
    pub max_connections: u32,
    pub secret_key: String,
}

pub fn get_config() -> Config {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap()
        .merge(config::Environment::new()).unwrap();

    // Print out our settings (as a HashMap)
    let config = settings.try_into::<Config>().unwrap();

    config 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_gets_config() {
        let config = get_config();
        assert_eq!("MY_KEY", config.secret_key);
        std::env::set_var("SECRET_KEY", "MY_KEY_2");
        let config = get_config();
        assert_eq!("MY_KEY_2", config.secret_key);
        std::env::remove_var("SECRET_KEY");
    }
}