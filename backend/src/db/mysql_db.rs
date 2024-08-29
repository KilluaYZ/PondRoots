use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;
use core::option::Option;
use actix_web::rt::time::Instant;
use crate::common::my_error::MyError;
// 用来处理日期

#[derive(Clone)]
pub struct MySqlDB{
    pool: Pool
}

#[derive(Debug)]
pub struct SqlExecResp {
    last_id: String,
}

impl MySqlDB {
    pub fn new(host: &str, port: u16, database: &str, username: &str, password: &str) -> MySqlDB {
        let url = format!("mysql://{}:{}@{}:{}/{}", username, password, host, port, database);
        println!("{}", url);
        let pool = Pool::new(&*url).unwrap();
        MySqlDB{
            pool
        }
    }

    pub fn get_conn(&self) -> PooledConn {
        self.pool.get_conn().unwrap()
    }

    pub fn query(&self, sql: String, params: Params) -> Result<Vec<Row>, MyError> {
        let mut conn = self.get_conn();
        let stmt = conn.prep(&sql).unwrap();
        match conn.exec(&stmt, &params){
            Ok(val) => Ok(val),
            Err(error) => {
                println!("Mysql query failed: {}, sql: {}", &error, &sql);
                Err(MyError::DetaildError(format!("Mysql query failed: {}, sql: {}", &error, &sql)))
            }
        }
    }

    pub fn query_first(&self, sql: String, params: Params) -> Result<Option<Row>, MyError> {
        let mut conn = self.get_conn();
        let stmt = conn.prep(&sql).unwrap();
        match conn.exec_first(stmt, params){
            Ok(val) => Ok(val),
            Err(error) => {
                println!("Mysql query_first failed: {}, sql: {}", &error, &sql);
                Err(MyError::DetaildError(format!("Mysql query failed: {}, sql: {}", &error, &sql)))
            }
        }
    }

    pub fn exec(&self, sql: String, params: Params) -> Result<u64, MyError>{
        let mut conn = self.get_conn();
        match conn.exec::<Row, &String, &Params>(&sql, &params){
            Ok(val) => Ok(conn.last_insert_id()),
            Err(e) => {
                println!("Mysql exec failed! sql: {}, error: {}", &sql, &e);
                Err(MyError::DetaildError(format!("Mysql query failed: {}, sql: {}", &e, &sql)))
            }
        }
    }

    pub fn start_transaction(&self) -> Transaction {
        self.pool.start_transaction(Default::default()).unwrap()
    }

    pub fn prep_exec(&self, sql: String, params: Params, mut transaction: &mut Transaction) -> Result<u64, MyError> {
        match transaction.exec::<Row, &String, &Params>(&sql, &params){
            Ok(val) => Ok(transaction.last_insert_id().unwrap()),
            Err(e) => {
                println!("Mysql prep_exec failed! sql: {}, error: {}", &sql, &e);
                Err(MyError::DetaildError(format!("Mysql query failed: {}, sql: {}", &e, &sql)))
            }
        }
    }

    pub fn commit_trans(&self, mut transaction: Transaction) -> Result<(), MyError>{
        match transaction.commit(){
            Ok(val) => Ok(val),
            Err(e) => {
                println!("MySql commit_trans fail: {}", e);
                Err(MyError::DetaildError(format!("MySql commit_trans fail: {}", e)))
            }
        }
    }

    pub fn rollback_trans(&self, mut transaction: Transaction) -> Result<(), MyError>{
        match transaction.rollback(){
            Ok(val) => Ok(val),
            Err(e) => {
                println!("MySql rollback_trans fail: {}", e);
                Err(MyError::DetaildError(format!("MySql commit_trans fail: {}", e)))
            }
        }
    }
}