use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use tokio::net::TcpListener;
use tracing::info;

use crate::utils::{regex::regex_defs::REGEX_LIST, stopwatch::stopwatch::Stopwatch};

use super::server_state_def::ServerState;

pub const DOMAIN_NAME: &'static str = "www.cyhdev.com";

pub const HOST_PORT_HTTP: u16 = 30737;
pub const HOST_ADDR_HTTP: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, HOST_PORT_HTTP));

pub async fn server_initializer(
    server_start_time: DateTime<Utc>,
    timer: &mut Stopwatch,
) -> Result<()> {
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

    // initialize server state
    let state = Arc::new(ServerState::new(server_start_time).await?);
    timer.click("Server state inititalized");

    // define routers here
    let main_router: axum::Router = axum::Router::new().with_state(Arc::clone(&state));
    timer.click("Routers defined");

    let listener: TcpListener = TcpListener::bind(HOST_ADDR_HTTP).await?;
    timer.total("Server started in");

    match axum::serve(
        listener,
        main_router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    {
        Ok(_) => (),
        Err(e) => {
            return Err(anyhow!("Axum could not serve app: {:?}", e));
        }
    };

    Ok(())
}
