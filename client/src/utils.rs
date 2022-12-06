use std::string::String;
use std::fmt::Debug;
use std::clone::Clone;
use std::string::ToString;
use std::str::FromStr;
use std::num::ParseIntError;


//消息结构体
#[derive(Debug,Clone)]
pub struct TextMessage{
    pub from : String, // 发送方地址
    pub to : String,// 接收方地址
    pub content : String,// 消息内容
    pub m_date : String // 消息时间
}

// 消息转换成字符串元组格式
impl ToString for TextMessage{
    fn to_string(&self) -> String{
        format!("({},{},{},{})",self.from,self.to,self.content,self.m_date)
    }
}

// 字符串元组转换成消息体
impl FromStr for TextMessage{
    type Err = ParseIntError;
    fn from_str(s : &str) -> Result<Self,Self::Err>{
        let message_info : Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' ).split(",").collect();
        let from = message_info[0].to_string();
        let to = message_info[1].to_string();
        let content = message_info[2].to_string();
        // let m_date = message_info[3].parse::<i64>()?;
        let m_date = message_info[3].to_string();
        Ok(TextMessage{from,to,content,m_date})
    }
}

// 留一些方法
mod util{
    // millis to data(yyyy-mm-dd hh:mm:ss)
}