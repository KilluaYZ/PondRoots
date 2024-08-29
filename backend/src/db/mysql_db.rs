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

    pub fn query(&self, sql: &str) -> Vec<Row> {
        let mut conn = self.get_conn();
        conn.query(sql).unwrap_or_else(|error| {
            println!("Mysql query failed: {}", error);
            vec![]
        })
    }

    pub fn query_first(&self, sql: &str) -> Option<Row> {
        let mut conn = self.get_conn();
        conn.query_first(sql).unwrap_or_else(|error| {
            println!("Mysql query_first failed: {}", error);
            None
        })
    }

    pub fn exec(&self, sql: &str, params: Params) -> u64{
        let mut conn = self.get_conn();
        let mut last_insert_id: u64 = 0;
        conn.exec_drop(sql, params).unwrap_or_else(|error| {
            println!("Mysql exec failed! sql: {}, error: {}", sql, error);
        });
        last_insert_id = conn.last_insert_id();
        last_insert_id
    }

    pub fn start_transaction(&self) -> Transaction {
        self.pool.start_transaction(Default::default()).unwrap()
    }

    pub fn prep_exec(&self, sql: &str, params: Params, mut transaction: &mut Transaction) -> u64 {
        let mut last_insert_id: u64 = 0;
        transaction.exec_drop(sql, params).unwrap_or_else(|error| {
            println!("Mysql exec failed! sql: {}, error: {}", sql, error);
        });
        last_insert_id = transaction.last_insert_id().unwrap();
        last_insert_id
    }

    pub fn commit_trans(&self, mut transaction: Transaction){
        transaction.commit().unwrap_or_else(|error| {
            println!("MySql commit_trans fail: {}", error);
        });
    }

    pub fn rollback_trans(&self, mut transaction: Transaction){
        transaction.rollback().unwrap_or_else(|error| {
            println!("MySql rollback_trans fail: {}", error);
        });
    }
}