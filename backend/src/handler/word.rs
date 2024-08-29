use std::ops::Index;
use actix_web::{post, web, Responder, HttpResponse};
use mysql::prelude::ToValue;
use mysql::Row;
use serde::{Deserialize, Serialize};
use crate::common::app_state::AppState;
use crate::common::resps::build_ok_response;
use crate::entity::word::*;
use mysql::from_value;

#[post("/word/get")]
pub async fn get_word_handler(req_body: String, data: web::Data<AppState>) -> impl Responder {
    println!("{}", req_body);
    let db = &data.db;
    let query_res = db.query("select * from word");
    let word_list = query_res.iter().map(|row| Word::new().from_row(row)).collect();
    build_ok_response::<Vec<Word>>(word_list)
}

#[post("/word/echo")]
pub async fn echo_word_handler(req_body: String) -> impl Responder {
    println!("req_body = {}", req_body);
    HttpResponse::Ok().body(req_body)
}