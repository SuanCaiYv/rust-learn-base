use std::net::{TcpListener, TcpStream};
use std::io::*;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;
use byteorder::ReadBytesExt;

struct Executors {
    thread_num: u32,
    worker_list: Vec<Worker>,
    sender: Sender<Job>,
}

impl Executors {
    fn new(core_size: u32) -> Self {
        let (sender, receiver) = channel();
        let receiver_lock = Mutex::new(receiver);
        let receiver_lock_clone = Arc::new(receiver_lock);
        let mut workers = Vec::new();
        for i in 0..core_size {
            let clone = receiver_lock_clone.clone();
            workers.push(Worker::new(format!("worker id: {}", i), clone));
        };
        Executors {
            thread_num: core_size,
            worker_list: workers,
            sender,
        }
    }

    fn execute<T>(&mut self, job: T)
        where T: FnOnce() + Send + 'static
    {
        self.sender.send(Box::new(job));
    }
}

fn do_job(job: Job) {
    job();
}

// 动态分发类型可以理解成一个指针，指向实际实现了这个trait的类型的对象
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: String,
    thread_handle: JoinHandle<()>,
}

impl Worker {
    fn new(id: String, clone: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread_handle = spawn(move || {
            loop {
                let rcv_guard = clone.lock().unwrap();
                let job = rcv_guard.recv().unwrap();
                do_job(job);
            }
        });
        Worker {
            id,
            thread_handle,
        }
    }
}

/// The Book最后的部分，也就不test_xxx()了吧
pub fn lite_web() {
    let mut executors = Executors::new(12);
    spawn(|| {
        sleep(Duration::from_millis(500));
        tcp_client();
    });
    let server_socket = TcpListener::bind("127.0.0.1:8190").unwrap();
    for stream in server_socket.incoming() {
        let mut socket = stream.unwrap();
        // 通过自定义线程池来实现多线程请求
        executors.execute(move || {
            loop {
                let mut len_buffer = [0 as u8; 4];
                let result = socket.read(&mut len_buffer);
                match result {
                    Ok(size) => {
                        let len = bytes_to_u32(&len_buffer);
                        if len == 0 {
                            break;
                        }
                        let mut buffer = Vec::new();
                        for i in 0..len {
                            buffer.push(socket.read_u8().unwrap());
                        }
                        let req = String::from_utf8(buffer).unwrap();
                        println!("服务端读到了: {}", &req);
                        let resp = format!("Hello, client. You have send me: {}, did you remember?", req);
                        socket.write(resp.as_bytes());
                        socket.flush();
                    }
                    Err(err) => {
                        break;
                    }
                }
            }
            println!("连接关闭")
        });
    }
}

// 大端法
fn bytes_to_u32(bytes: &[u8; 4]) -> u32 {
    ((bytes[3] as u32) << 24) +
        ((bytes[2] as u32) << 16) +
        ((bytes[1] as u32) << 8) +
        ((bytes[0] as u32) << 0)
}

fn tcp_client() {
    for idx in 0..50 {
        let handle = spawn(move || {
            sleep(Duration::from_micros((50-idx)/3));
            let mut socket = TcpStream::connect("127.0.0.1:8190").unwrap();
            for i in 0..5 {
                let mut input = idx.to_string() + " : " + &i.to_string();
                let mut req = Vec::new();
                // 计算长度
                let len = input.len() as u32;
                // 基于长度的TCP
                for b in len.to_ne_bytes() {
                    req.push(b);
                }
                for b in input.as_bytes() {
                    req.push(*b);
                }
                socket.write(req.as_slice());
                socket.flush();
                input.clear();
                let mut buffer = [0 as u8; 1024];
                let size = socket.read(&mut buffer).unwrap();
                let text = String::from_utf8_lossy(&buffer[0..size]);
                println!("{}", text);
            }
        });
    }
    sleep(Duration::from_secs(10));
    // let mut socket = TcpStream::connect("127.0.0.1:8190").unwrap();
    // let scanner = stdin();
    // let mut input = String::new();
    // println!("input 5 loops....");
    // for i in 0..5 {
    //     scanner.read_line(&mut input);
    //     input.pop();
    //     let mut req = Vec::new();
    //     let len = input.len() as u32;
    //     for b in len.to_ne_bytes() {
    //         req.push(b);
    //     }
    //     for b in input.as_bytes() {
    //         req.push(*b);
    //     }
    //     socket.write(req.as_slice());
    //     socket.flush();
    //     input.clear();
    //     let mut buffer = [0 as u8; 1024];
    //     let size = socket.read(&mut buffer).unwrap();
    //     let text = String::from_utf8_lossy(&buffer[0..size]);
    //     println!("{}", text);
    // }
}