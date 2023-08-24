use std::{thread::{self, JoinHandle}, sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}};

pub mod explanation;

type Job = Box<dyn FnOnce() + 'static + Send>;

struct Worker {
    thread: JoinHandle<()>,
    id: usize,
}

impl Worker {
    fn new(reciever: Arc<Mutex<Receiver<Job>>>, id: usize) -> Worker {
        let thread = thread::spawn(move || {
            let job = reciever.lock().unwrap().recv().unwrap();

            job();
        });

        Worker {
            thread,
            id
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        assert!(num_threads > 0);

        let (tx, rx) = mpsc::channel();
        
        let rx: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(rx));

        let workers = (0..num_threads).into_iter().map(move |id| Worker::new(rx.clone(), id)).collect();

        ThreadPool { workers, sender: tx}
    }

    pub fn enter<F>(&mut self, task: F) where
        F: FnOnce() + 'static + Send  
    {
        self.sender.send(Box::new(task));
    }
}
