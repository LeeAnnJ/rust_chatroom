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

use super::{UserInfo, SimpleUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqID {
    pub id: i32
}

impl ReqID {
    // 登录的数据库操作
    pub async fn search_user_id (self, pool: &Pool<MySql>) -> Result<UserInfo, sqlx::Error>{
        let sql = format!("select * from users where ID={};",self.id);
        let res = sqlx::query(&sql)
            .fetch_all(pool)
            .await;
        match res {
            Ok(rows) => {
                if rows.len()>0 {
                    let user = UserInfo{ 
                        ID: rows[0].get("ID"),
                        PW: rows[0].get("PW"),
                        uName: rows[0].get("uName")
                    };
                    return Ok(user);
                }
                else {return Err(sqlx::Error::RowNotFound) };
            },
            Err(e) => {
                println!("sql:{}\n get database query bug",sql);
                return Err(e);
            }
        };
    }

    // 获取好友列表
    pub async fn get_friend_list (self, pool: &Pool<MySql>) -> Result<Vec<SimpleUser>, sqlx::Error>{
        let sql = format!("select rID as ID, uName from friends FS,users US where sID={} and FS.rID=US.ID;",self.id);
        let res = sqlx::query(&sql)
            .fetch_all(pool)
            .await;
        let mut friends = vec![];
        match res {
            Ok(rows) => {
                for row in rows {
                    friends.push( SimpleUser { 
                        ID: row.get("ID"),
                        uName: row.get("uName")
                    });
                }
                return Ok(friends);
            },
            Err(e) => {
                println!("sql:{}\n get database query bug",sql);
                return Err(e);
            }
        };
    }
}
