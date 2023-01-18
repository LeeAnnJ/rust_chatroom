use std::{convert::Infallible, io, env};
use actix::{Actor,StreamHandler};
use actix_files::{Files, NamedFile};
use actix_web::{
    get, post, App, HttpRequest, HttpServer, web, error, HttpResponse, middleware, Either, Responder, Result, 
    web::Data,
    http::{
        header::{self, ContentType},
        Method, StatusCode,
    },
};
use actix_web_actors::ws::{self, Message, WebsocketContext};
use serde::Deserialize;
use serde_json::json;
use async_stream::stream;

// extern crate chrono;
extern crate dotenv;
extern crate sqlx;

use dotenv::dotenv;
use sqlx::{
    mysql::{MySqlPoolOptions, MySql},
    Pool,Error,FromRow, Row
};

struct AppState {
    pool: Pool<MySql>
}

#[derive(Debug, Deserialize)]
pub struct User {
    id: i32,
    name: String,
    password: String,
}

impl User {
    async fn login (self, pool: &Pool<MySql>) -> Result<i32, sqlx::Error>{
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
}

// 使用结构体接收
#[get("/user/login/{name}/{pass}")]
async fn login(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse{
    let user = User{
        id: 0,
        name: req.match_info().query("name").parse().unwrap(),
        password: req.match_info().query("pass").parse().unwrap()
    };
    println!("{:?}",user);
    //"ok".to_string()
    //format!("{:?},{:},{:}",user,user.name,user.password)
    let pool = &&data.pool;
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

#[actix_web::main]
async fn start_actix() -> std::io::Result<()> {
    let db_pool = match start_database().await {
        Ok(pool) => pool,
        Err(e) => {
            println!("error link database: {:?}", e);
            return Err(std::io::Error::last_os_error());
        },
    };
    
    // 创建一个json解析配置，并用于处理json解析
    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::Conflict().finish())
                .into()
        });

    println!("starting HTTP server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            
            // 这个配置只会影响json解析，不会影响其他类型的解析
            .app_data(json_config.clone())
            .app_data(web::Data::new(AppState {
                pool: db_pool.clone(),
            }))
            .service(login)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// 启动数据库连接池
async fn start_database() -> Result<Pool<MySql>, Error>{
    dotenv().ok(); // 检测并读取.env文件中的内容，若不存在也会跳过异常
    let url = &std::env::var("DATABASE_URL").unwrap();
    println!("database url:{:?}",url);
    let pool = MySqlPoolOptions::new()
        .max_connections(100)  // 连接池的上限
        //.min_connections(10)   // 连接池的下限
        .acquire_timeout(std::time::Duration::from_secs(10))    // 连接超时时间
        .max_lifetime(std::time::Duration::from_secs(1800))     // 所有连接的最大生命周期
        .idle_timeout(std::time::Duration::from_secs(600))      // 空闲连接的生命周期
        .connect(url)
        .await;
    pool
}

fn main(){
    start_actix().unwrap();
}