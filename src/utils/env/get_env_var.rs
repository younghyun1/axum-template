use anyhow::{anyhow, Result};
use dotenvy::var;

pub fn get_env_var(key: &str) -> Result<String> {
    match var(key) {
        Ok(value) => Ok(value),
        Err(e) => Err(anyhow!("Could not find var {}: {:?}", key, e)),
    }
}
