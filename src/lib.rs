use std::{thread, sync::{mpsc, Arc, Mutex}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Message>();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // Must use smart thread-safe smart pointers
            // since workers have shared ownership and mutability of receiver 
            workers.push(Worker::new(
                id, 
                Arc::clone(&receiver)
            ));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // Will fail if threads stop running but
        // we will always want our threads running to accept requests
        self.sender.send(Message::NewJob(job)).unwrap(); 
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Send terminate message to all threads
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        println!("Shutting down all workers");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize, 
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();
            
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                },
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker { id, thread: Some(thread) }
    }
}
