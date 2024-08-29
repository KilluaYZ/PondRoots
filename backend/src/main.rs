use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use chrono::NaiveDate;
use mysql::{from_row, params};
use mysql::prelude::Queryable;
use pond_roots::common::app_state::AppState;
use pond_roots::db::mysql_db::{MySqlDB};
use pond_roots::db::init_db::*;
use pond_roots::handler::word::*;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let db = MySqlDB::new(
        "killuayz.top",
        47365,
        "pond_roots",
        "root",
        "bgBpTS6RgNmdh34dUgE534sT2G73KaBSNsWxC0qdlkg3pTWRfB"
    );
    let app_state = web::Data::new(
        AppState{
            db
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_word_handler)
            .service(echo_word_handler)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
