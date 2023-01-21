use serde::{Serialize, Deserialize};
use actix_web:: Result;

extern crate sqlx;
use sqlx::{
    mysql::MySql,
    Pool, Row
};

use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RecieveMessage {
    mesID: i32,
    sID: i32,
    rID: i32,
    mes: String,
    sTime: NaiveDateTime,
    isread: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relation {
    pub user: i32,
    pub friend: i32
}

impl Clone for Relation {
    fn clone(&self) -> Self {
        Self { user: self.user.clone(), friend: self.friend.clone() }
    }
}

impl Copy for Relation {}

impl Relation {
    // 查询是否为好友
    async fn check_relation (self, pool: &Pool<MySql>) -> Result<bool, sqlx::Error>{
        let sql = format!("select * from friends where sID={} and rID={};",self.user,self.friend);
        let res = sqlx::query(&sql)
            .fetch_all(pool)
            .await;
        match res {
            Ok(rows) => {
                if rows.len()>0 {
                    return Ok(true);
                }
                else {return Ok(false)};
            },
            Err(e) => {
                println!("sql:{}\n get database query bug",sql);
                return Err(e);
            }
        };
    }

    // 建立好友关系
    pub async fn create_relation (self, pool: &Pool<MySql>) -> Result<u64, sqlx::Error>{
        let check = self.check_relation(pool).await.unwrap();
        if !check {
            let sql = format!("insert into friends(sID,rID) values({0},{1}),({1},{0});",self.user,self.friend);
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
        } else {
            return Ok(0);
        }
    }

    // 获取聊天记录
    pub async fn get_record (self, pool: &Pool<MySql>, unread: bool) -> Result<Vec<RecieveMessage>, sqlx::Error>{
        let mut sql = format!("SELECT * from meslog where ((sID={0} AND rID={1}) OR (sID={1} AND rID={0}))",self.user,self.friend);
        if unread {sql+=" and isread = 0"}
        sql+=" ORDER BY sTime desc;";
        let res = sqlx::query(&sql)
            .fetch_all(pool)
            .await;
        let mut meslist = vec![];
        match res {
            Ok(rows) => {
                for row in rows {
                    let i:i64 = row.get("isread");
                    meslist.push( RecieveMessage {
                        mesID: row.get("mesID"),
                        sID: row.get("sID"),
                        rID: row.get("rID"),
                        mes: row.get("mes"),
                        sTime: row.get("sTime"),
                        isread: if i==0 {false} else {true}
                    })
                }
                return Ok(meslist);
            },
            Err(e) => {
                println!("sql:{}\n get database query bug",sql);
                return Err(e);
            }
        };
    }
}