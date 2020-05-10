//! 线程池实现
//! 主要实施: Work工作模块 threadPool线程池
//!

use std::thread;
use std::sync::{Arc,Mutex,mpsc};

struct Work {
    // id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Work {
    fn new(id: usize,receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Work {
        let th = thread::spawn(move ||{
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::Job(job) => {
                        println!("Worker {} receive a job",id);
                        // job.call_box();
                        job();
                    },
                    Message::Terminate => {
                        println!("Worker {} terminate",id);
                        break;
                    },
                }
            }
        });
        Work{
            // id,
            thread:Some(th),
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl <F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

enum Message {
    Job(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Work>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (rx,rp) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(rp));
        for id in 0..size {
            workers.push(Work::new(id,Arc::clone(&receiver)));
        }

        ThreadPool{
            workers,
            sender:rx,
        }
    }
    pub fn exec<F>(&self,f:F)
        where F:FnOnce() + Send + 'static
    {
        self.sender.send(Message::Job(Box::new(f))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // awit
        for work in &mut self.workers {
            if let Some(work) = work.thread.take() {
                work.join().unwrap();
            }
        }
    }
}
