use std::sync::OnceLock;

use crate::{error::Result, support::env::get_env};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env().unwrap_or_else(|err| panic!("Fatal - Could not load configuration: {err:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub DATABASE_URL: String,
}

impl Config {
    pub fn load_from_env() -> Result<Self> {
        Ok(Self {
            DATABASE_URL: get_env("DATABASE_URL")?,
        })
    }
}