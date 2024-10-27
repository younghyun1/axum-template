use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use dotenvy::var;
use tracing::info;

use crate::utils::{regex::regex_defs::REGEX_LIST, stopwatch::stopwatch::Stopwatch};

pub async fn server_initializer(server_start_time: DateTime<Utc>) -> Result<()> {
    let mut timer = Stopwatch::new(Some("Validating regexes..."));

    // validate regexes
    for regex in REGEX_LIST {
        let start = tokio::time::Instant::now();
        match regex.is_valid() {
            Ok(_) => {
                info!("{} validated in {:?}", regex.name, start.elapsed());
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    timer.click("Regexes validated");
    let app_name_version: String = get_env_var("APP_NAME_VERSION")?;
    let db_name: String = get_env_var("DB_NAME")?;
    let db_password: String = get_env_var("DB_PASSWORD")?;
    let db_port: String = get_env_var("DB_PORT")?;
    let db_host: String = get_env_var("DB_HOST")?;
    let db_user: String = get_env_var("DB_USER")?;
    
    

    Ok(())
}

fn get_env_var(key: &str) -> Result<String> {
    match var(key) {
        Ok(value) => Ok(value),
        Err(e) => Err(anyhow!("Could not find var {}: {:?}", key, e)),
    }
}
