use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

fn handle_echo_client(mut stream: TcpStream) {
    // 50 byte 接收数据
    let mut data = [0 as u8; 50];
    // 读取传过来的数据
    while match stream.read(&mut data) {
        Ok(size) => {
            // 传过来的数据反向传回去
            stream.write(&data[0..size]).unwrap();
            true
        }
        Err(_) => {
            // 无法读取则打印错误并关闭stream流
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    // TcpListener 监听 TCP 的连接，并监听 0.0.0.0:3333 端口
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();

    // 处理连接流
    for stream in listener.incoming() {
        // stream连接
        match stream {
            // 连接成功将流下传处理
            Ok(stream) => {
                // 打印连接的IP
                println!("New connection: {}", stream.peer_addr().unwrap());
                // 处理消息
                handle_echo_client(stream)
            }
            Err(e) => {
                // 连接有问题则打印错误
                println!("Error: {}", e);
            }
        }
    }

    // 关闭连接
    drop(listener);
}

