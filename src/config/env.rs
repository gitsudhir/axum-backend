use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_port")]
    pub port: u16,
    pub database_url: Option<String>,
}

fn default_port() -> u16 {
    3000
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        Ok(Config {
            port: cfg.get_int("PORT").unwrap_or(3000) as u16,
            database_url: cfg.get_string("DATABASE_URL").ok(),
        })
    }
}