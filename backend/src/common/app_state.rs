use std::sync::Arc;
use crate::db::mysql_db::MySqlDB;

#[derive(Clone)]
pub struct AppState{
    pub db: MySqlDB
}