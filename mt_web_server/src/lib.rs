use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// `num_workers` is the number of workers in the pool.
    ///
    ///  # Panics
    ///
    /// The `new` function will panic if `num_workers` is zero
    pub fn new(num_workers: usize) -> ThreadPool {
        assert!(num_workers > 0);

        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(num_workers);

        for id in 0..num_workers {
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        ThreadPool {
            workers,
            sender: tx,
        }
    }

    pub fn exec<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Terminating all workers...");
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.handle.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = rx.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job. Executing...", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} terminating now...", id);
                    break;
                }
            }
        });

        Worker {
            id,
            handle: Some(thread),
        }
    }
}
