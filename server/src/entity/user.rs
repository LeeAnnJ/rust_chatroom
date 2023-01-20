use std::ptr::null;

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
    mysql::{MySqlPoolOptions, MySql},
    Pool,Error,FromRow, Row
};

// 使用于请求体的用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

// 完整的用户信息结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo{
    pub ID: i32,
    pub PW: String,
    pub uName: String
}

// 返回给客户端的用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleUser {
    pub ID: i32,
    pub uName: String
}

impl User {
    // 登录的数据库操作
    pub async fn login (self, pool: &Pool<MySql>) -> Result<i32, sqlx::Error>{
        let sql = format!("select * from users where uName='{}' and PW='{}';",self.name,self.password);
        let res = sqlx::query(&sql)
            .fetch_all(pool)
            .await;
        match res {
            Ok(rows) => {
                if rows.len()>0 {
                    let id = rows[0].get("ID");
                    return Ok(id);
                }
                else {return Ok(0)};
            },
            Err(e) => {
                println!("sql:{}\n get database query bug",sql);
                return Err(e);
            }
        };
    }
    // 注册的数据库操作
    pub async fn registor (self, pool: &Pool<MySql>) -> Result<i32, sqlx::Error>{
        let sql = format!("insert into users(PW,uName) values('{}','{}');",self.password,self.name);
        let res = sqlx::query(&sql)
            .execute(pool)
            .await;
        match res {
            Ok(affect) => {
                let affect_row = affect.rows_affected();
                if affect_row>0 {
                    let id = self.login(&pool.clone()).await.unwrap();
                    return Ok(id);
                }
                else {
                    println!("failed to insert a user row!");
                    return Err(sqlx::Error::RowNotFound)
                };
            },
            Err(e) => {
                println!("sql:{}\n get database query bug",sql);
                return Err(e);
            }
        };
    }
}