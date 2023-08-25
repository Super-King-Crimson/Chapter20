use std::{thread::{self, JoinHandle}, sync::{mpsc::{self, Receiver, Sender}, Arc, Mutex}};

pub mod explanation;

type Job = Box<dyn FnOnce() + 'static + Send>;

struct Worker {
    thread: Option<JoinHandle<()>>,
}

impl Worker {
    fn new(reciever: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {                
                let job = match reciever.lock().unwrap().recv() {
                    Ok(val) => val,
                    Err(_) => {
                        break;
                    }
                };

                job();
            }
        });

        Worker {
            thread: Some(thread)
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        assert!(num_threads > 0);

        let (tx, rx) = mpsc::channel();
        
        let rx: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(rx));

        let workers = (0..num_threads).into_iter().map(move |_| Worker::new(rx.clone())).collect();

        ThreadPool { workers, sender: Some(tx)}
    }

    pub fn enter<F>(&mut self, task: F) where
        F: FnOnce() + 'static + Send  
    {
        self.sender.as_ref().unwrap().send(Box::new(task)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        //Hang up the transmitter thread so receivers stop waiting for messages
        drop(self.sender.take());

        for worker in &mut self.workers {
            //join with the thread to let it finish what it's doing
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}