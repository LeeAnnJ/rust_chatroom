use std::net::TcpListener;
use std::ptr::null;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::io::{ErrorKind,Read,Write};
use std::time::Duration;
// use chrono::prelude::*;

mod utils;
// use server::utils::TextMessage;

const LOCAL_HOST:&str ="localhost:8000"; //监听地址和端口，先试试这样行不行
const MESSAGE_SIZE:usize = 1024; // 信息的缓冲区大小

// 休眠功能
fn sleep(){ 
    thread::sleep(Duration::from_millis(100));
}

fn main() {
    // 创建TCP监听器
    let listener =TcpListener::bind(LOCAL_HOST)
        .expect("Failed to create TCPListener");
    // 设置非阻塞模式 
    listener.set_nonblocking(true)
        .expect("Cannot set non-blocking");
    // 保存所有连接到服务端的客户端
    let mut clients = vec![];
    // 实例化信道，信道传输的数据类型设置为String
    let (sender,receiver) = mpsc::channel::<String>();
    
    loop{
        // 接收连接 (TCPStream,SocketAddr)
        if let Ok((mut socket,address)) = listener.accept() {
            let port = address.port();
            println!("Client port {} Connected.",port);
            // 向数组插入客户端对象
            clients.push(socket.try_clone().expect("Failed to clone client"));
            
            // 开始接收消息
            // 复制一个消息队列的生产者
            let sd = sender.clone();
            // 创建子线程
            thread::spawn(move || loop{
                // 创建一个消息缓存区
                let mut buffer =vec![0;MESSAGE_SIZE];
                // 读取TCP流中的消息
                match socket.read_exact(&mut buffer){
                    Ok(_) =>{
                        // 从缓冲区读取信息
                        let message = buffer.into_iter()
                            .take_while(|&x| x!=0)
                            .collect::<Vec<_>>();
                        // 将信息转换为utf8格式
                        let message = String::from_utf8(message)
                            .expect("Invalid utf8 message");
                        // 将消息发送到消息队列
                        sd.send(message)
                            .expect("Failed to send message to receiver");
                    },
                    // 阻塞错误
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    // 其他错误
                    Err(_) =>{
                        // 在这里处理
                        // 结束线程
                        break;
                    }
                }
                // 线程休眠
                sleep();
            });
        }
        
        // 开始转发消息部分
        // 从队列获取消息
        if let Ok(message) = receiver.try_recv(){ 
            let msg = message.clone();
            if !msg.is_empty() {
                println!("Receive message: [{}]",msg);
                // 转发给每一个客户端
                clients =clients.into_iter().filter_map(|mut client|{
                    // 将消息队列放入缓冲区
                    let mut buffer = message.clone().into_bytes();
                    buffer.resize(MESSAGE_SIZE,0);
                    client.write_all(&buffer).map(|_| client).ok()
                }).collect::<Vec<_>>();
            }
        }
        sleep();
    }
}
