extern crate sqlx;
use sqlx::{
    mysql::MySql,
    Pool,
};

pub struct AppState {
    pub pool: Pool<MySql>
}