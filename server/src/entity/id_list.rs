use serde::{Serialize, Deserialize};
use actix_web::Result;

extern crate sqlx;
use sqlx::{
    mysql::MySql,
    Pool
};

#[derive(Debug, Serialize, Deserialize)]
pub struct IDList {
    pub ids: Vec<i32>
}

impl IDList {
    // 生成批量修改的sql需要的字符串
    fn crate_ctring (self, read: bool) -> String {
        let state = if read {1} else {0};
        let mut str = format!("UPDATE meslog SET isread={0} WHERE mesID in ({1}",state,self.ids[0]);
        let len = self.ids.len();
        for i in 1..len{
            str += &(format!(",{}",self.ids[i]));
        }
        str += ");";
        str
    }
    pub async fn set_read (self, pool: &Pool<MySql>, read: bool) -> Result<u64, sqlx::Error>{
        let sql = self.crate_ctring(read);
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
    }
}