use std::sync::RwLock;

use chrono_tz::Tz;
use deadpool_postgres::Pool;

use super::shuffle_bag::ShuffleBag;

pub struct ServerState {
    pub timezone: Tz,
    pub app_name_version: String,
    pub pool: Pool,
    pub quote_shuffle_bag: RwLock<ShuffleBag>,
}
