use std::cmp::PartialEq;
use std::ops::Index;
use actix_web::cookie::time::Date;
use std::error::Error;
use mysql::{from_value, Row, Value, params};
use serde::{Deserialize, Serialize};
use crate::common::my_error::MyError;
use crate::db::mysql_db::MySqlDB;
use crate::entity::roots_affixes_derive::RootsDerive;
use crate::entity::roots_affixes_form::RootsForm;
use core::option::Option;

#[derive(Deserialize, Serialize)]
pub struct RootsAffixes{
    id: Option<u32>,
    roots_derive: Option<RootsDerive>,
    derive_text: Option<String>,
    form: Option<Vec<RootsForm>>,
    zh_interpretation: Option<String>,
    en_interpretation: Option<String>,
    roots_affixes_type: Option<u32>,
    create_time: Option<String>,
    update_time: Option<String>,
}

impl RootsAffixes{
    pub fn new() -> RootsAffixesBuilder{
        RootsAffixesBuilder{
            roots_affixes: RootsAffixes{
                id: None,
                roots_derive: None,
                derive_text: None,
                form: None,
                zh_interpretation: None,
                en_interpretation: None,
                roots_affixes_type: None,
                create_time: None,
                update_time: None,
            }
        }
    }
}


pub struct RootsAffixesBuilder {
    roots_affixes: RootsAffixes
}

impl RootsAffixesBuilder {
    pub fn id(mut self, id: u32) -> Self {
        self.roots_affixes.id = Option::from(id);
        self
    }

    pub fn roots_derive(mut self, roots_derive: RootsDerive) -> Self {
        self.roots_affixes.roots_derive = Option::from(roots_derive);
        self
    }

    pub fn derive_text(mut self, derive_text: String) -> Self {
        self.roots_affixes.derive_text = Option::from(derive_text);
        self
    }

    pub fn form(mut self, form: Vec<RootsForm>) -> Self {
        self.roots_affixes.form = Option::from(form);
        self
    }

    pub fn zh_interpretation(mut self, zh_interpretation: String) -> Self {
        self.roots_affixes.zh_interpretation = Option::from(zh_interpretation);
        self
    }

    pub fn en_interpretation(mut self, en_interpretation: String) -> Self {
        self.roots_affixes.en_interpretation = Option::from(en_interpretation);
        self
    }

    pub fn roots_affixes_type(mut self, roots_affixes_type: u32) -> Self {
        self.roots_affixes.roots_affixes_type = Option::from(roots_affixes_type);
        self
    }

    pub fn create_time(mut self, create_time: Date) -> Self {
        self.roots_affixes.create_time = Option::from(create_time.to_string());
        self
    }

    pub fn update_time(mut self, update_time: Date) -> Self {
        self.roots_affixes.update_time = Option::from(update_time.to_string());
        self
    }

    pub fn from_row(self, row: Row, db: &MySqlDB) -> Result<RootsAffixes, MyError> {
        let mut builder = RootsAffixes::new();

        // 先获取一些可以直接得到的属性
        let id = row.index(0).clone();
        let derive_text = row.index(2).clone();
        let zh_interpretation = row.index(3).clone();
        let en_interpretation = row.index(4).clone();
        let roots_affixes_type = row.index(5).clone();
        let create_time = row.index(6).clone();
        let update_time = row.index(7).clone();

        // 下面开始获取一些比较复杂的属性
        let derive_from_id = from_value::<u32>(row.index(1).clone());
        let roots_derive = match RootsDerive::new().from_id(derive_from_id, &db){
            Ok(val) => val,
            Err(e) => {
                return Err(e)
            }
        };

        let form = match RootsForm::new().from_roots_affixes_id(from_value::<u32>(id.clone()), &db){
            Ok(val) => val,
            Err(e) => {
                return Err(e)
            }
        };

        if id != Value::NULL{
            builder = builder.id(from_value::<u32>(id));
        }

        builder = builder.roots_derive(roots_derive);

        if derive_text != Value::NULL{
            builder = builder.derive_text(from_value::<String>(derive_text));
        }

        builder = builder.form(form);

        if zh_interpretation != Value::NULL{
            builder = builder.zh_interpretation(from_value::<String>(zh_interpretation));
        }

        if en_interpretation != Value::NULL{
            builder = builder.en_interpretation(from_value::<String>(en_interpretation));
        }

        if roots_affixes_type != Value::NULL{
            builder = builder.roots_affixes_type(from_value::<u32>(roots_affixes_type));
        }

        if create_time != Value::NULL{
            builder = builder.create_time(from_value::<Date>(create_time));
        }
        if update_time != Value::NULL{
            builder = builder.update_time(from_value::<Date>(update_time));
        }

        Ok(builder.build())
    }

    pub fn from_id(self, id: u32, db: &MySqlDB) -> Result<RootsAffixes, MyError> {
        let query_res = match db.query(
            String::from("select * from roots_affixes where id = :id"),
            params! {"id" => id}
        ){
            Ok(val) => val,
            Err(e) => {
                return Err(e)
            }
        };

        if query_res.len() == 0{
            return Err(MyError::DetaildError(format!("roots_affixes::from_id失败！id为{}的词根词缀不存在！", id)))
        };
        RootsAffixes::new().from_row(query_res[0].clone(), db)
    }

    pub fn from_word_id(self, word_id: u32, db: &MySqlDB) -> Result<Vec<RootsAffixes>, MyError> {
        let roots_id_list = match db.query(
            String::from("select roots_affixes_id from word_roots_affixes where word_id = :word_id order by order_no"),
            params! {"word_id" => word_id}
        ){
            Ok(val) => val,
            Err(e) => {
                return Err(e)
            }
        };

        let mut roots_affixes_list = Vec::<RootsAffixes>::new();
        for item in roots_id_list{
            let roots_affixes_id = from_value(item.index(0).clone());
            let tmp_roots = match RootsAffixes::new().from_id(roots_affixes_id, db){
                Ok(val) => val,
                Err(e) => {
                    return Err(e)
                }
            };
            roots_affixes_list.push(tmp_roots);
        };

        Ok(roots_affixes_list)
    }

    pub fn build(self) -> RootsAffixes { self.roots_affixes }
}