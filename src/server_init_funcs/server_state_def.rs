use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use deadpool_postgres::Config;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::Object;
use deadpool_postgres::Pool;
use tokio_postgres::NoTls;

use crate::utils::env::get_env_var::get_env_var;
use crate::utils::stopwatch::stopwatch::Stopwatch;

pub struct ServerState {
    timezone: Tz,
    server_start_time: DateTime<Utc>,
    app_name_version: String,
    pool: Pool,
}

impl ServerState {
    pub async fn new(server_start_time: DateTime<Utc>) -> Result<ServerState> {
        let mut timer = Stopwatch::new(None);

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

        timer.click("server configurations loaded");

        let pool: Pool = db_config.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)?;
        let conn = pool.get().await?;

        timer.click("connection pool initialized, connection pulled");

        let ver_string = conn
            .query_one("SELECT VERSION();", &[])
            .await
            .map_err(|e| anyhow!("Failed to execute test query on the database: {:?}", e))?
            .get::<usize, String>(0);

        timer.click(&format!("DB connection validated; {}; latency", ver_string));

        Ok(ServerState {
            timezone,
            server_start_time,
            app_name_version,
            pool,
        })
    }

    pub async fn get_conn(self) -> Result<Object> {
        self.pool.get().await.map_err(|e| e.into())
    }

    pub fn get_version(self) -> String {
        self.app_name_version
    }

    pub fn get_server_start_time(self) -> DateTime<Utc> {
        self.server_start_time
    }

    pub fn get_timezone(self) -> Tz {
        self.timezone
    }
}
