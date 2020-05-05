# 无畏并发

### 基础理论
- 1. 进程是资源分配的最小单位 线程是CPU调度的最小单位
- 2. 使用多线程时的问题:
    - 1. 竞争状态: 多个线程以不一致的顺序访问资源或数据
    - 2. 死锁: 两个线程互相等待对象停止使用其所拥有的资源 造成两者永久等待
        - 栗子: A线程 手上有 1资源 2资源 需要得到3资源才会释放1,2  B线程 拥有3资源 需要得到1,2资源才会释放4
- 3. 编程语言提供协程叫做绿色线程，如GO 在底层实现来M：N模型
    - M个绿色线程(协程) 对应N个OS线程  但是 RUST 只提供1:1线程模型实现 即一改Rust线程对于一个Os线程
    - 运行时代表二进制文件中包含对语言本身的提供代码，这些代码更具语言的不同可大可小，不过非汇编语言都会有一定数量的运行时代码  通常大家说一个语言"没有运行时“ 是值这个语言”运行时“很小  Rust,C都是几乎没有运行时
    
### easy dome test1
``` 
use std::thread;
use std::time::Duration;

pub fn test1() {
    let s1 = thread::spawn(hello);
    thread::spawn(hello2);
    thread::spawn(||{
        thread::sleep(Duration::from_secs(1));
        panic!("Thread 3 Panic"); // 子线程Panic 主线程不会Panic
    });

    for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    s1.join().unwrap();
    println!("Over");
}

fn hello2() {
    let mut i = 0;
    loop {
        if i < 1000 {
            i += 1;
            thread::sleep(Duration::from_millis(500));
            println!("S2 i: {}",i);
            continue;
        }
        break
    }
}

fn hello() {
    let mut i = 0;
    loop {
        if i < 1000 {
            i += 1;
            thread::sleep(Duration::from_millis(1));
            println!("S1 i: {}",i);
            continue;
        }
        break
    }
}
```

### 涉及作用域所有权 test2
``` 
use std::thread;
use std::time::Duration;

pub fn test2() {
    let ips = vec!["0.0.0.0","1.1.1.1","2.2.2.2"];
    let a = thread::spawn(move ||{
        // for i in ips {
        //     println!("Ic: {}",i);
        // }
        thread::sleep(Duration::from_secs(1));
        println!("ips: {:?}",ips);
        // rust 不知道ips 的生命周期长度
        // 无法保证ips 始终有效   所有需要移交所有权给它
    });

    a.join().unwrap()
}
```

### 消息传递 test3
> Rust 实现消息传递并发的主要工具是通道。 由两部分组成 一改发送端 一该接受端。 
> 发送 或则 接受端 任意一个被丢弃时 视作通道被关闭
- 通道
    - 1. 通过`mpsc::channel` 创建通道 mpsc 多个生产者 单一消费者
    - 2. 通过`spmc::channel` 创建通道 spmc 一个生产者 多个消费者
- 创建通道后返回发生者 和 消费者
``` 
let (tx,rx) = mpsc::channel();
let (tx,rx) = spmc::channel();
```
demo: 
``` 
use std::sync::mpsc;
use std::thread;

pub fn test3() {
    let (tx,rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");

        match tx.send(val) {
            Ok(_) => println!("send success"),
            Err(e) => println!("send err: {}",e),
        }
        // 发送 移交所有权
    });


    let received = rx.recv().unwrap();
    // 接受数据 获得所有权   recv() 如果 生产者全部死了 就会 返回错误  反之就会阻塞一直等待数据

    println!("Get: {}",received);
}
```
#### 注意 
- 发送者send 会返回一改Result<T,E>
- 如果接受端被丢弃来 没有发送目标 此时发送会返回错误
- 接受者recv会返回一改Result 当发送端被丢弃是 会返回一个错误值  反之一直等待数据
- `recv()` 接受会阻塞等待  `try_recv()` 不会阻塞 立即返回

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn test4() {
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(250));
        }
    });

    loop {
        match rx.recv() {
            Ok(d) => println!("data: {}",d),
            Err(e) => {
                println!("err: {}",e);
                break;
            },
        }
    }

}
```

#### mpsc demo test5
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn test5() {
    let (tx,rx) = mpsc::channel();
    // 增加一个生产者
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        for i in 0..99 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::spawn(move || {
        for i in 100..999 {
            tx2.send(i).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in rx {
        println!("Resp: {}",i);
    }
}
```

## 共享内存  多所有权 test6
这里会引入两个智能指针
- `Mutex<T>` 提供多线程下内部可变型  类似与`RefCell<T>`
- `Arc<T>` 提供多线程下共享  类似 `Rc<T>`
- `RefCell<T>/Rc<T>` 是非线性安全的，`Mutex<T>/Arc<T>` 是线程安全的
```rust
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
// use std::time::Duration;

pub fn test6() {
    let a = Arc::new(Mutex::new(0));
    let mut li = vec![];
    for _i in 0..100 {
        let ic = Arc::clone(&a);
        let th = thread::spawn(move || {
            *ic.lock().unwrap() += 1;
            // loop {
            //     thread::sleep(Duration::from_millis(1))
            // }
        });
        li.push(th);
    }

    for i in li {
        i.join().unwrap()
    }

    println!("Over: {}",*a.lock().unwrap())
}
```
## Send Sync trait test7
有两个并发概念内嵌与语言中: `std::marker` 中的 `Synv` 和 `Send trait`
- 通过Send 允许在线程间转移所有权
    - Send 标记 trait 表示类型所有权可以在线程间传递  几乎所有Rust类型都是Send的 但是例外 如`Rc<T>` 不能Send的
    - 任何完全由Send类型组成也会自动标记为Send
- Sync 运行多线程访问
    - 1. Sync标记 trait 表明一个实现Sync的类型可以安全的在多个线程中拥有其值的引用  即 对于任意类型T 如果&T(T的引用)是Send的话T就是Sync的  着意味着其引用就可以安全的发送到另一个线程
    - 2. 智能指针 `Rc<T>` 也不是Sync的 出于其不是Send相同的原因 RefCell<T> 和Cell<T> 类型不是Sync的 RefCell<T> 在运行时所进行借用检查也不是线程安全的 Mutex<T> 是Sync的
- 手动实现Send和Sync是不安全的
    - 通常并不需要手动实现Send 和 Sync trait 因为由Send 和Sync的类型组成的类型 自动就是Send和Sync的 因为它们是标记trait 甚至都不需要实现方法  它们只是用来加强并发相关的不可变性的
    