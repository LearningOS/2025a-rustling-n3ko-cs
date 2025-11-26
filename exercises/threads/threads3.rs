// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// 直接接收完整 Queue 和原始 Sender，内部克隆 Sender 给两个线程
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> (thread::JoinHandle<()>, thread::JoinHandle<()>) {
    // 克隆 Sender 给第一个线程（多生产者核心：通过 clone() 实现）
    let tx1 = tx.clone();
    let handle1 = thread::spawn(move || {
        for val in q.first_half {
            println!("sending {:?}", val);
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 直接使用原始 Sender 给第二个线程（也可再 clone，效果一致）
    let handle2 = thread::spawn(move || {
        for val in q.second_half {
            println!("sending {:?}", val);
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    (handle1, handle2)  // 返回两个线程句柄，供主线程等待
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    // 启动发送线程，获取句柄
    let (handle1, handle2) = send_tx(queue, tx);

    // 等待所有发送线程完成（确保所有数据发送完毕，Sender 全部 drop 后通道关闭）
    handle1.join().unwrap();
    handle2.join().unwrap();

    // 单消费者：主线程接收所有数据
    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);  // 10 == 10，断言成功
}
