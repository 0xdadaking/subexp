///
/// 第3课作业 TCP Server
/// @author 王大大
/// @date 2022-7-9 16:30
///
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

///输入流处理函数
fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    // 分配64字节数组buffer，用于读取输入流
    let mut buffer = [0 as u8; 64];
    // 读数据到buffer数组，
    while match stream.read(&mut buffer) {
        // 有读取到长度为size的数据
        Ok(size) => {
            if size > 0 {
                // 模拟耗时处理，等待1秒才回写
                thread::sleep(Duration::from_secs(1));
                // 将读取的数据原样写回
                stream.write(&buffer[0..size])?;
                // match表达式为true，将继续循环读取
                true
            } else {
                //如果读到的数据长度为0，则说明数据发送已经结束或连接关闭
                false
            }
        }
        // 读取数据出错
        Err(_) => {
            println!(
                "An error occurred, terminating connection with {}",
                stream.peer_addr()?
            );
            // 关闭流
            stream.shutdown(Shutdown::Both)?;
            // match表达式为false，将退出循环
            false
        }
    } {}
    println!("Exit handler_client() for peer: {}", stream.peer_addr()?);
    Ok(())
}

fn main() {
    // 监听所有Network Interface的1234端口
    let listener = TcpListener::bind("0.0.0.0:1234").unwrap();
    println!("Server listening on port 1234");
    // 接受连接(listener.incoming()调用将阻塞，直到有连接进入)，并为每个连接开启对应线程来处理
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // 连接成功
                println!("New connection: {}", stream.peer_addr().unwrap());
                // 开线程处理连接的流, stream被move
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                // 连接出错
                println!("Error: {}", e);
            }
        }
    }
    // 主动关闭socket server
    drop(listener);
    println!("Server exit successful!");
}
