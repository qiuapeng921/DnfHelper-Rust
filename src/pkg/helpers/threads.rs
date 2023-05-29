use std::thread;
use std::thread::sleep_ms;

fn test() {
    // 创建一个新的线程
    let handle = thread::spawn(|| {
        loop {
            println!("Thread {}", 1);
            sleep_ms(500);
            // 检查是否需要强制终止线程
            if thread::panicking() {
                return;  // 线程被强制终止
            }
        }
    });

    // 等待 2 秒后停止线程
    sleep_ms(2000);

    // 终止线程
    handle.thread().unpark();
    drop(handle);  // 等待线程退出并释放资源
}