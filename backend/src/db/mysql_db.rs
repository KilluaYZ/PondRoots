use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;
use core::option::Option;

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

    pub fn query(&self, sql: String, params: Params) -> Result<Vec<Row>, Error> {
        let mut conn = self.get_conn();
        let stmt = conn.prep(sql)?;
        match conn.exec(stmt, params){
            Ok(val) => Ok(val),
            Err(error) => {
                println!("Mysql query failed: {}, sql: {}", &error, &sql);
                Err(error)
            }
        }
    }

    pub fn query_first(&self, sql: String, params: Params) -> Result<Option<Row>, Error> {
        let mut conn = self.get_conn();
        let stmt = conn.prep(sql)?;
        match conn.exec_first(stmt, params){
            Ok(val) => Ok(val),
            Err(error) => {
                println!("Mysql query_first failed: {}, sql: {}", &error, &sql);
                Err(error)
            }
        }
    }

    pub fn exec(&self, sql: String, params: Params) -> Result<u64, Error>{
        let mut conn = self.get_conn();
        let mut last_insert_id: u64 = 0;
        match conn.exec(&sql, &params){
            Ok(val) => Ok(conn.last_insert_id()),
            Err(e) => {
                println!("Mysql exec failed! sql: {}, error: {}", &sql, &e);
                Err(e)
            }
        }
    }

    pub fn start_transaction(&self) -> Transaction {
        self.pool.start_transaction(Default::default()).unwrap()
    }

    pub fn prep_exec(&self, sql: String, params: Params, mut transaction: &mut Transaction) -> Result<u64, Error> {
        match transaction.exec(&sql, &params){
            Ok(val) => Ok(transaction.last_insert_id().unwrap_or_else(|e| {0})),
            Err(e) => {
                println!("Mysql prep_exec failed! sql: {}, error: {}", &sql, &e);
                Err(e)
            }
        }
    }

    pub fn commit_trans(&self, mut transaction: Transaction) -> Result<(), Error>{
        match transaction.commit(){
            Ok(val) => Ok(val),
            Err(e) => {
                println!("MySql commit_trans fail: {}", e);
                Err(e)
            }
        }
    }

    pub fn rollback_trans(&self, mut transaction: Transaction) -> Result<(), Error>{
        match transaction.rollback(){
            Ok(val) => Ok(val),
            Err(e) => {
                println!("MySql rollback_trans fail: {}", e);
                Err(e)
            }
        }
    }
}