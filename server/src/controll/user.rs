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

use crate::appState::AppState;
use crate::entity::ReqID;

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

// 登录接口，使用结构体接收
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
                    "status":0,
                    "data": {
                        "result": true,
                        "id": result
                    }
                })
            )
    }
    else {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(
                json!({
                    "status": 0,
                    "data":{"result": false},
                    "msg": "fail to add uer info to database!"
                })
            )
    }
}

// 注册接口
#[post("/user/registor")]
pub async fn registor(form: web::Json<User>, data: Data<AppState>) -> HttpResponse{
    let user = form.into_inner();
    println!("in registor function:{:?}",user);
    let pool = &data.pool;
    let name = Text{name: (&user.name).to_string()};
    let res = name.search_name(pool).await.unwrap();
    if res.len()>0 {
        return HttpResponse::Ok()
            .content_type(ContentType::json())
            .json( json!({
                "status": 400,
                "data":{
                    "result": false,
                },
                "msg": "account name has been used!"
            }))
    }

    let result = user.registor(pool).await.unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json( json!({
            "status": 0,
            "data":{
                "result": true,
                "id": result
            }
        }))
}

// 完整的用户信息结构体
#[derive(Debug, Deserialize, Serialize)]
pub struct UserInfo{
    pub ID: i32,
    pub PW: String,
    pub uName: String
}

#[derive(Debug, Deserialize)]
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
}

// 根据用户名查询信息 接口方法 
#[get("/user/getUserByName")]
pub async fn search_user_by_name(req: web::Query<Text>, data: Data<AppState>) -> HttpResponse {
    let name=req.into_inner();
    println!("get user name:{:?}",name);
    let pool = &data.pool;
    let result = name.search_name(pool).await.unwrap();
    let res = if result.len()>0 {
        json!({
            "status": 0,
            "data":{
                "users": result,
            }
        })
    } else {
        json!({
            "status": 0,
            "data":{
                "users": [],
            },
            "msg":"no such name."
        })
    };
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(res)
}

// 根据用户id返回用户
#[get("/user/getUserById")]
pub async fn search_user_by_id(req: web::Query<ReqID>, data: Data<AppState>) -> HttpResponse {
    let id = req.into_inner();
    println!("get user by id:{:?}",id);
    let pool = &data.pool;
    let result = id.search_user_id(pool).await;
    let res = match result {
        Ok(user) => {
            json!({
                "status": 0,
                "data": {
                    "user": user
                },
            })
        },
        Err(e) => {
            json!({
                "status": 400,
                "data":{"user": null },
                "msg": "can't find user in database!"
            })
        }
    };
    HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(res)
}