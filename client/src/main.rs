use std::net::TcpStream;
use std::io::{self,ErrorKind,Read,Write};
use std::sync::mpsc::{self,TryRecvError};
use std::time::Duration;
use std::thread;
use std::str;

mod utils;

const LOCAL_HOST:&str = "localhost:8000"; // 服务端地址
const MESSAGE_SIZE:usize = 1024; //缓冲区大小

// 休眠功能
fn sleep(){ 
    thread::sleep(Duration::from_millis(100));
}

fn main(){
    //连接服务端
    let mut client = TcpStream::connect(LOCAL_HOST)
        .expect("Failed to connect");
    // 设置非阻塞
    client.set_nonblocking(true)
        .expect("Failed to intiate non-bolcking");
    // 实例化信道，信道传输的数据类型设置为String
    let (sender,receiver) = mpsc::channel::<String>();
    
    // 接收消息部分
    thread::spawn(move || loop{
        let mut buffer = vec![0;MESSAGE_SIZE];
        match client.read_exact(&mut buffer){
            Ok(_) =>{
                let message = buffer.into_iter()
                    .take_while(|&x| x!=0)
                    .collect::<Vec<_>>();
                let message_string = str::from_utf8(&message).unwrap();
                println!("Receive: {}",message_string);
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock =>(),
            Err(_) =>{
                println!("Connection with server was served");
                break;
            }
        }
        match receiver.try_recv() {
            Ok(message) =>{
                let mut buffer = message.clone().to_string().into_bytes();
                buffer.resize(MESSAGE_SIZE,0);
                client.write_all(&buffer).expect("Writing to socket failed");
            },
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) =>break
        }
    });

    println!("*********************************");
    println!("************ WELCOME ************");
    println!("*********************************");

    // 不断等待从终端读取消息,使用"exit"退出
    loop{ 
        let mut buffer = String::new();
        // 读取用户在命令行发送的消息
        io::stdin().read_line(&mut buffer)
            .expect("Failed to read from stdin");
        let message = buffer.trim().to_string();
        let mut msg = message.clone();
        if message == "exit" || sender.send(message).is_err() {
            break;
        }

        // 为消息创建消息体
        // let text_message = utils::TextMessage{
        //     from: String::from("(send_address)"),
        //     to: String::from("(receive_address)"),
        //     content: message.clone(),
        //     m_date: Utc::now().to_string();
        // };

        if sender.send(msg.clone()).is_err(){
            break;
        }
        println!("Send Message: [{}]",msg);
    }

    println!("*********************************");
    println!("************ GOODBYE ************");
    println!("*********************************");
}