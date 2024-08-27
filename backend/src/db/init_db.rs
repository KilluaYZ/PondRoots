use crate::db::mysql_db::MySqlDB;
use std::fs::File;
use std::io::{self,Read};

pub fn init_db_tables(db: MySqlDB) {
    let path = "src/db/pond_roots.sql";
    let mut file = File::open(path);
    let mut contents = String::new();
    file.unwrap().read_to_string(&mut contents);
    println!("文件内容: {}", contents);
}