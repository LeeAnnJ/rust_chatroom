use serde::{Serialize, Deserialize};
use actix_web::Result;

extern crate sqlx;
use sqlx::{
    mysql::MySql,
    Pool, Row
};

use super::{SimpleUser,UserInfo};

#[derive(Debug, Deserialize, Serialize)]
pub struct Text{
    pub name: String
}

impl Text {
    // 根据用户名精准查询用户信息
    pub async fn search_name (self, pool: &Pool<MySql>) -> Result<Vec<UserInfo>, sqlx::Error>{
        let mut users = vec![];

        let sql = format!("select * from users where uName='{}';",self.name);
        let res = sqlx::query(&sql)
            .fetch_all(pool)
            .await;
        match res {
            Ok(rows) => {
               for row in rows {
                    users.push(UserInfo {
                        ID: row.get("ID"),
                        PW: row.get("PW"),
                        uName: row.get("uName")
                    })
               }
               return Ok(users);
            },
            Err(e) => {
                println!("sql:{}\n get database query bug",sql);
                return Err(e);
            }
        };
    }

    // 根据字符串模糊查询用户信息
    pub async fn search_user (self, pool: &Pool<MySql>) -> Result<Vec<SimpleUser>, sqlx::Error>{
        let mut users = vec![];
        let x = match self.name.parse::<i32>(){Ok(v) => v,Err(_) => 0};

        let sql = format!("select ID,uName from users where uName like '%{0}%' or ID={1};",self.name,x);
        let res = sqlx::query(&sql)
            .fetch_all(pool)
            .await;
        match res {
            Ok(rows) => {
               for row in rows {
                    users.push(SimpleUser {
                        ID: row.get("ID"),
                        uName: row.get("uName")
                    })
               }
               return Ok(users);
            },
            Err(e) => {
                println!("sql:{}\n get database query bug",sql);
                return Err(e);
            }
        };
    }
}