use std::thread::*;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::sync::{Mutex, Arc};
use std::borrow::Borrow;

pub fn test_thread() {
    let handle1 = spawn(|| {
        println!("thread started!")
    });
    handle1.join();
    let vec1 = vec![1, 2, 3];
    // move强制转移所有权到新的线程中
    let handle2 = spawn(move || {
        for v in vec1.iter() {
            println!("{}", v)
        }
    });
    handle2.join();

    let (producer, consumer) = channel();
    let handle3 = spawn(move || {
        for i in 0..10 {
            // 创建一个字符串
            let str = String::from(i.to_string());
            // 由于send()调用，所有权转移到channel中
            producer.send(str);
            sleep(Duration::from_millis(1000));
        };
    });
    for val in consumer.iter() {
        // 迭代循环会调用recv()并在无数据时阻塞，recv()返回值带有所有权
        // 此时所有权转移到了这里
        println!("{}", val)
    }
    handle3.join();

    // 创建一个互斥量
    let lock0 = Mutex::new(12);
    // 此外，想要在多个线程中共享锁，需要每个线程都获得锁对象所有权，但是Rc<>不适合并发场景，因为设计原子更新问题
    // 这里我们使用线程安全的Arc<>来实现。
    let lock = Arc::new(lock0);
    let mut handles = Vec::new();
    for i in 0..5 {
        let lock_inner = lock.clone();
        let handle = spawn(move || {
            // 在Rust中，锁持有和所有权绑定，锁定的值离开作用域，自动释放锁，理解成自动回收，用完了自然要释放嘛～
            let mut num = lock_inner.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles.into_iter() {
        handle.join();
    }
    let num = *lock.lock().unwrap();
    println!("{}", num);
}