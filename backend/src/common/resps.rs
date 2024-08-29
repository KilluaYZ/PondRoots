use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CommonResp<T>{
    msg: String,
    code: u32,
    data: T
}

fn build_resp<T: Serialize>(code:u32, msg: String, resp_data: T) -> String{
    let common_resp = CommonResp::<T>{
        code: code,
        msg: msg,
        data: resp_data
    };
    serde_json::to_string(&common_resp).unwrap()
}

pub fn build_ok_response<T: Serialize>(resp_data: T) -> HttpResponse {
    HttpResponse::Ok().body(build_resp::<T>(200, String::from("操作成功"), resp_data))
}

pub fn build_internal_server_error_response<T: Serialize>(resp_data: T) -> HttpResponse {
    HttpResponse::InternalServerError().body(build_resp::<T>(500, String::from("服务器内部错误"), resp_data))
}

pub fn build_bad_request_response<T: Serialize>(resp_data: T) -> HttpResponse {
    HttpResponse::BadRequest().body(build_resp::<T>(400, String::from("请求参数错误"), resp_data))
}