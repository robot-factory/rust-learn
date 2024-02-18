# 异步

## 并行
普通的并行一般使用多线程
```
use std::thread::sleep;
use std::thread;
use std::time::Duration;
use chrono::Local;

fn read_from_file1() -> String {
    sleep(Duration::new(4, 0));
    "Hello from file1".to_string()
}

fn read_from_file2() -> String {
    sleep(Duration::new(2, 0));
    "Hello from file2".to_string()
}

fn main() {
    let mut now = Local::now();
    println!("{}", now.to_rfc3339());
    let handler1 = thread::spawn(|| {
        let content1 = read_from_file1();
        println!("{}", content1);
    });
    let handler2 = thread::spawn(|| {
        let content2 = read_from_file2();
        println!("{}", content2);
    });
    handler1.join().unwrap();
    handler2.join().unwrap();
    now = Local::now();
    println!("{}", now.to_rfc3339());
}
```

## 并发


## tokio入门