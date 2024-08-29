use std::fmt::format;
use std::ops::Index;
use actix_web::{post, web, Responder, HttpResponse};
use mysql::prelude::ToValue;
use mysql::{params, Row};
use serde::{Deserialize, Serialize};
use crate::common::app_state::AppState;
use crate::common::resps::{build_bad_request_response, build_internal_server_error_response, build_ok_response};
use crate::entity::word::*;
use crate::entity::word_get_request::*;
use mysql::from_value;
use serde::de::Unexpected::Option;

#[post("/word/get")]
pub async fn get_word_handler(req_body: String, data: web::Data<AppState>) -> impl Responder {
    let request =  match serde_json::from_str::<WordGetRequest>(&*req_body){
        Ok(val) => val,
        Err(error) => {
            // 如果json解析失败，则提前结束，返回错误resp
            return build_bad_request_response::<Vec<u32>>(vec![0], format!("json串：{} 解析错误，错误原因：{}", req_body, error))
        }
    };
    let db = &data.db;
    let query_res = match db.query(
        format!("select * from word where american_word = :american_word or british_word = :british_word"),
        params! {"british_word"=> &request.word, "american_word"=> &request.word}
    ){
        Ok(val) => val,
        Err(error) => {
            // 如果json解析失败，则提前结束，返回错误resp
            return build_internal_server_error_response::<Vec<u32>>(vec![0], format!("服务器内部错误，错误原因：{}", error))
        }
    };
    let mut word_list = Vec::<Word>::new();

    for row in query_res{
        let tmp_word = match Word::new().from_row(row, db){
            Ok(val) => val,
            Err(e) => {
                return build_internal_server_error_response::<Vec<u32>>(vec![0], format!("服务器内部错误，错误原因：{}", e))
            }
        };
        word_list.push(tmp_word);
    }

    build_ok_response::<Vec<Word>>(word_list)
}