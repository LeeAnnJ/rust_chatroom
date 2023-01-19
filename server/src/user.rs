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

use crate::appState::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub password: String,
}

impl User {
    // 登录的数据库操作
    pub async fn login (self, pool: &Pool<MySql>) -> Result<i32, sqlx::Error>{
        let sql = format!("select * from users where uName='{}' and PW='{}';",self.name,self.password);
        let res = sqlx::query(&sql)
            .bind(self.name)
            .bind(self.password)
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

// 使用结构体接收
#[get("/user/login/{name}/{pass}")]
pub async fn login(req: HttpRequest, data: Data<AppState>) -> HttpResponse{
    let user = User{
        name: req.match_info().query("name").parse().unwrap(),
        password: req.match_info().query("pass").parse().unwrap()
    };
    println!("login: {:?}",user);
    //format!("{:?},{:},{:}",user,user.name,user.password)
    let pool = &data.pool;
    let result = user.login(pool).await.unwrap();
    if result>0 {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(
                json!({
                    "result": true,
                    "id": result
                })
            )
    }
    else {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(
                json!({"result": false})
            )
    }
}

#[post("/user/registor")]
pub async fn registor(form: web::Json<User>, data: Data<AppState>) -> HttpResponse{
    // let user = User{
    //     name: req.match_info().query("name").parse().unwrap(),
    //     password: req.match_info().query("pass").parse().unwrap()
    // };
    let user = form.into_inner();
    println!("in registor function:{:?}",user);
    // format!("{:?},{:},{:}",user,user.name,user.password)
    let pool = &data.pool;
    let result = user.registor(pool).await.unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json( json!({
            "result": true,
            "id": result
        }))
}