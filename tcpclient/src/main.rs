///
/// 第3课作业 TCP Client
/// @author 王大大
/// @date 2022-7-9 16:30
///
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::thread;
use std::time::Duration;

fn main() {
    //TCP连接localhost:1234地址
    match TcpStream::connect("localhost:1234") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 1234");
            //模拟连续发送5次消息
            for _ in 1..5 {
                let msg = b"Hello!";
                //向流写入数据
                stream.write(msg).unwrap();
                println!("Sent Hello, awaiting reply...");

                let mut data = [0 as u8; 6];
                //等待服务端回写数据，只读取发出的6字节
                match stream.read_exact(&mut data) {
                    Ok(_) => {
                        if &data == msg {
                            println!("Reply is ok!");
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply: {}", text);
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
                //模拟等待
                thread::sleep(Duration::from_secs(2));
            }
        }
        Err(e) => {
            // 打印出错信息
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
