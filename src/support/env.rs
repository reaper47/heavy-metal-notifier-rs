use std::env;

use crate::error::{Error, Result};

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name)
        .map(|v| v.trim_matches('"').to_string())
        .map_err(|_| Error::MissingEnv(name))
}
