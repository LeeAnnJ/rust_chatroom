use serde::{Serialize, Deserialize};
use actix_web::{
    get, post, HttpRequest, HttpServer, web, error, HttpResponse, middleware, Either, Responder, Result, 
    web::Data,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
};
use serde_json::json;
use async_stream::stream;

extern crate sqlx;
use sqlx::{
    mysql::{ MySql },
    Pool,Error,FromRow, Row
};

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
}