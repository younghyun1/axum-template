use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use deadpool_postgres::Config;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::Pool;
use tokio_postgres::NoTls;

use crate::utils::env::get_env_var::get_env_var;

pub struct ServerState {
    timezone: Tz,
    server_start_time: DateTime<Utc>,
    app_name_version: String,
    pool: Pool,
}

impl ServerState {
    pub fn new(server_start_time: DateTime<Utc>) -> Result<ServerState> {
        let app_name_version: String = get_env_var("APP_NAME_VERSION")?;
        let timezone_str: String = get_env_var("TIMEZONE")?;
        let timezone: Tz = timezone_str
            .parse()
            .map_err(|_| anyhow!("Invalid timezone"))?;

        // db configuration from env
        let mut db_config: Config = Config::new();
        db_config.user = Some(get_env_var("DB_USER")?);
        db_config.host = Some(get_env_var("DB_HOST")?);
        db_config.dbname = Some(get_env_var("DB_NAME")?);
        db_config.password = Some(get_env_var("DB_PASSWORD")?);
        db_config.port = Some(get_env_var("DB_PORT")?.parse()?);
        db_config.manager = Some(ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Fast, // look into more later
        });

        Ok(ServerState {
            timezone,
            server_start_time,
            app_name_version,
            pool: db_config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?,
        })
    }
}
