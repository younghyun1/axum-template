use anyhow::anyhow;
use chrono::{DateTime, Utc};
use dotenvy::dotenv;
use server_init_funcs::server_init::server_initializer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use utils::stopwatch::stopwatch::Stopwatch;
pub mod server_init_funcs {
    pub mod server_init;
}
pub mod utils {
    pub mod stopwatch {
        pub mod stopwatch;
    }
}

// async executor per thread iirc
#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    // time measurements
    let mut stopwatch = Stopwatch::new(Some("Initializing webserver..."));
    let server_start_time: DateTime<Utc> = Utc::now();

    // adjust logging levels here
    let mut filter: EnvFilter =
        EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?; //for prod

    // exclude output from external crates here
    filter = filter
        .add_directive("axum-template=info".parse()?)
        .add_directive("rustls=off".parse()?)
        .add_directive("aws_config=off".parse()?);

    // set logging parameters here
    tracing_subscriber::fmt()
        .with_ansi(false) // disable colored output; advisable if persisting logs to external
        // .with_target(false) // disable target display
        .with_env_filter(filter)
        .init();

    // load .env files
    match dotenv() {
        Ok(path_buf) => {
            stopwatch.click(&format!(
                "Env. variables at {} loaded",
                path_buf.to_str().unwrap_or("N/A"),
            ));
        }
        Err(e) => {
            return Err(anyhow!(
                "Dotenvy could not load .env file: {}",
                e.to_string()
            ));
        }
    }

    // server initialization logic separated for potential future unit testing.
    match server_initializer(server_start_time).await {
        Ok(_) => {
            info!("webserver successfully terminated",);
            return Ok(());
        }
        Err(e) => {
            return Err(anyhow!("{:?}", e));
        }
    }

    Ok(())
}
