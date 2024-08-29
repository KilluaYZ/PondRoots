use serde::{Deserialize, Serialize};
use mysql::{from_value, params, Error, Row, Value};
use actix_web::cookie::time::Date;
use core::option::Option;
use std::ops::Index;
use crate::common::my_error::MyError;
use crate::db::mysql_db::MySqlDB;

#[derive(Deserialize, Serialize, Clone)]
pub struct RootsForm {
    id: Option<u32>,
    roots_affixes_id: Option<u32>,
    form: Option<String>,
    create_time: Option<String>,
    update_time: Option<String>,
}

impl RootsForm {
    pub fn new() -> RootsFormBuilder{
        RootsFormBuilder{
            roots_form: RootsForm{
                id: None,
                roots_affixes_id: None,
                form: None,
                create_time: None,
                update_time: None
            }
        }
    }
}

pub struct RootsFormBuilder {
    roots_form: RootsForm
}

impl RootsFormBuilder {
    pub fn id(mut self, id: u32) -> Self {
        self.roots_form.id = Option::from(id);
        self
    }

    pub fn roots_affixes_id(mut self, roots_affixes_id: u32) -> Self {
        self.roots_form.roots_affixes_id = Option::from(roots_affixes_id);
        self
    }

    pub fn form(mut self, form: String) -> Self {
        self.roots_form.form = Option::from(form);
        self
    }

    pub fn create_time(mut self, create_time: Date) -> Self {
        self.roots_form.create_time = Option::from(create_time.to_string());
        self
    }

    pub fn update_time(mut self, update_time: Date) -> Self {
        self.roots_form.update_time = Option::from(update_time.to_string());
        self
    }

    pub fn from_row(self, row: &Row) -> RootsForm {
        let mut builder = RootsForm::new();
        let id = row.index(0).clone();
        let roots_affixes_id = row.index(1).clone();
        let form = row.index(2).clone();
        let create_time = row.index(3).clone();
        let update_time = row.index(4).clone();

        if id != Value::NULL{
            builder = builder.id(from_value::<u32>(id));
        }

        if roots_affixes_id != Value::NULL{
            builder = builder.roots_affixes_id(from_value::<u32>(roots_affixes_id));
        }

        if form != Value::NULL{
            builder = builder.form(from_value::<String>(form));
        }

        if create_time != Value::NULL{
            builder = builder.create_time(from_value::<Date>(create_time));
        }
        if update_time != Value::NULL{
            builder = builder.update_time(from_value::<Date>(update_time));
        }
        builder.build()
    }

    pub fn from_roots_affixes_id(self, roots_affixes_id: u32, db: &MySqlDB) -> Result<Vec<RootsForm>, MyError>{
        let query_res = match db.query(
            String::from("select * from roots_affixes_form where roots_affixes_id = :roots_affixes_id order by form"),
            params! {"roots_affixes_id" => roots_affixes_id}
        ){
            Ok(val) => val,
            Err(e) => {
                println!("roots_affixes_form::from_roots_affixes_id 失败! from_roots_affixes_id为{}, 错误为{}", &roots_affixes_id, &e);
                return Err(e)
            }
        };

        let form_list = query_res.iter().map(|row| RootsForm::new().from_row(row)).collect();
        Ok(form_list)
    }

    pub fn build(self) -> RootsForm {
        self.roots_form
    }
}