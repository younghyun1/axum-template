use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Utc};
use tracing::info;

use crate::utils::{regex::regex_defs::REGEX_LIST, stopwatch::stopwatch::Stopwatch};

use super::server_state_def::ServerState;

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

    let state = Arc::new(ServerState::new(server_start_time)?);
    
    
    Ok(())
}
