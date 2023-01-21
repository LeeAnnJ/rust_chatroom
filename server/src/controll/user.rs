use actix_web::{
    get, post, HttpRequest, web, HttpResponse,
    web::Data,
    http::{
        header::ContentType,
    },
};
use serde_json::json;

use crate::appState::AppState;
use crate::entity::{ReqID, Text, User};

// 登录接口，使用结构体接收 已测试
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

// 注册接口 已测试
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

// 根据用户名查询信息 接口方法  已测试
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

// 根据用户id返回用户 已测试
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
        Err(_) => {
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

// 模糊查询接口 已测试
#[get("user/search")]
pub async fn search_user_fuzzy(req: web::Query<Text>, data: Data<AppState>) -> HttpResponse {
    let text = req.into_inner();
    println!("serch user:{:?} by name or id",text);
    let pool = &data.pool;
    let result = text.search_user(pool).await;
    let res = match result {
        Ok(list) => {
            json!({
                "status": 0,
                "data": {
                    "user": list
                },
            })
        },
        Err(_) => {
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