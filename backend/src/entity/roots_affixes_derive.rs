use std::fmt::format;
use std::ops::Index;
use mysql::{from_value, params, Error, Row, Value};
use serde::{Deserialize, Serialize};
use crate::common::my_error::MyError;
use crate::db::mysql_db::MySqlDB;

#[derive(Deserialize, Serialize, Clone)]
pub struct RootsDerive{
    id: Option<u32>,
    en_full: Option<String>,
    en_abrev: Option<String>,
    zh_full: Option<String>,
    zh_abrev: Option<String>,
}

impl RootsDerive{
    pub fn new() -> RootsDeriveBuilder{
        RootsDeriveBuilder{
            roots_derive: RootsDerive{
                id: None,
                en_full: None,
                en_abrev: None,
                zh_full: None,
                zh_abrev: None
            }
        }
    }
}

pub struct RootsDeriveBuilder{
    roots_derive: RootsDerive
}

impl RootsDeriveBuilder {
    pub fn id(mut self, id: u32) -> Self {
        self.roots_derive.id = Option::from(id);
        self
    }

    pub fn en_full(mut self, en_full: String) -> Self {
        self.roots_derive.en_full = Option::from(en_full);
        self
    }

    pub fn en_abrev(mut self, en_abrev: String) -> Self {
        self.roots_derive.en_abrev = Option::from(en_abrev);
        self
    }

    pub fn zh_full(mut self, zh_full: String) -> Self {
        self.roots_derive.zh_full = Option::from(zh_full);
        self
    }

    pub fn zh_abrev(mut self, zh_abrev: String) -> Self {
        self.roots_derive.zh_abrev = Option::from(zh_abrev);
        self
    }

    pub fn build(self) -> RootsDerive {
        self.roots_derive
    }

    pub fn from_row(self, row: &Row) -> RootsDerive{
        let mut roots_derive = RootsDerive::new();
        let id = row.index(0).clone();
        let en_full = row.index(1).clone();
        let en_abrev = row.index(2).clone();
        let zh_full = row.index(3).clone();
        let zh_abrev = row.index(4).clone();

        if id != Value::NULL{
            roots_derive = roots_derive.id(from_value::<u32>(id));
        }
        if en_full != Value::NULL{
            roots_derive = roots_derive.en_full(from_value::<String>(en_full));
        }
        if en_abrev != Value::NULL{
            roots_derive = roots_derive.en_abrev(from_value::<String>(en_abrev));
        }
        if zh_full != Value::NULL{
            roots_derive = roots_derive.zh_full(from_value::<String>(zh_full));
        }
        if zh_abrev != Value::NULL{
            roots_derive = roots_derive.zh_abrev(from_value::<String>(zh_abrev));
        }
        roots_derive.build()
    }

    pub fn from_id(self, id: u32, db: &MySqlDB) -> Result<RootsDerive, MyError>{
        let query_res = match db.query(
            String::from("select * from roots_derives where id = :id"),
            params! {"id" => id}
        ){
            Ok(val) => val,
            Err(e) => {
                println!("roots_affixes_derive::from_id 失败! id{}, 错误为{}", &id, &e);
                return Err(e)
            }
        };

        let derive_list:Vec<RootsDerive> = query_res.iter().map(|row| RootsDerive::new().from_row(row)).collect();
        Ok(derive_list[0].clone())
    }
}