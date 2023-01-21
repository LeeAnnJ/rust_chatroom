use serde::{Serialize, Deserialize};
use actix_web::Result;

extern crate sqlx;
use sqlx::{
    mysql::{ MySql },
    Pool,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessage{
    pub send: i32,
    pub recieve: i32,
    pub message: String,
    pub isRead: bool
}

impl SendMessage {
    // 增加消息记录
    pub async fn add_record (self, pool: &Pool<MySql>) -> Result<u64, sqlx::Error>{
        let read = if self.isRead {1} else {0};
        let sql = format!("insert into meslog(sID,rID,mes,sTime,isread) values({},{},\"{}\",NOW(),{});",self.send,self.recieve,self.message,read);
        let res = sqlx::query(&sql)
            .execute(pool)
            .await;
        match res {
            Ok(affect) => {
                let affect_row = affect.rows_affected();
                return Ok(affect_row)
            },
            Err(e) => {
                println!("sql:{}\n get database query bug",sql);
                return Err(e);
            }
        };
    }
}