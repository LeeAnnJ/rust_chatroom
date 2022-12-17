// extern crate chrono;
extern crate dotenv;
extern crate sqlx;

// use chrono::NaiveDateTime;
// use chrono::{Datelike, Duration, Local, TimeZone, Timelike};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
use std::io;


#[derive(Debug)]
// pub struct meslog {
//     // pub mesID: i32, // 数据库中将此字段设置为了自增主键，插入时无需使用
//     pub sID: i32,   // 不存在空值，不需使用Option来处理潜在的空值情况
//     pub rID: i32,
//     pub Mes: String,
//     // pub sTime: DateTime<Local>,  插入时数据库会调用Now，无需在rust里调用
// }
#[db::main]
pub async fn main() -> io::Result<()>{
    dotenv().ok();  // 检测并读取.env文件中的内容，若不存在也会跳过异常

    let pool = sqlx::MySqlPool::builder().
        max_size(100).  // 连接池的上限
        min_size(10).   // 连接池的下限
        connect_timeout(std::time::Duration::from_secs(10)).    // 连接超时时间
        max_lifetime(std::time::Duration::from_secs(1800)).     // 所有连接的最大生命周期
        idle_timeout(std::time::Duration::from_secs(600)).      // 空闲连接的生命周期
        build(&std::env::var("DATABASE_URL").unwrap()).await?;

    let mut conn = pool.acquire().await?;
    let sID = 1;    //改为传入参数
    let rID = 2;    //改为传入参数
    let Mes = "hello--";    //改为传入参数
    let sql = format!(r#"insert into meslog(sID,rID,mes,sTime) values({},{},{}},NOW());"#,sID,rID,Mes); 
    let affect_rows = sqlx::query(sql).execute(&mut conn).await?;
    println!("{:?}", affect_rows);

    drop(conn);
    Ok(())
}