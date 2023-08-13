// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.





use std::sync::{mpsc, Arc, Mutex};
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

fn send_tx(q: Arc<Mutex<Queue>>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc1 = Arc::clone(&q);
    let tc1 = Arc::clone(&tx);
    let thread1 = thread::spawn(move || {
        let queue = qc1.lock().unwrap();
        let sender = tc1.lock().unwrap();
        for val in &queue.first_half {
            println!("sending {:?}", val);
            sender.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let qc2 = Arc::clone(&q);
    let tc2 = Arc::clone(&tx);
    let thread2 = thread::spawn(move || {
        let queue = qc2.lock().unwrap();
        let sender = tc2.lock().unwrap();
        for val in &queue.second_half {
            println!("sending {:?}", val);
            sender.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Arc::new(Mutex::new(Queue::new()));
    let sender = Arc::new(Mutex::new(tx));
    let queue_length = queue.lock().unwrap().length;
    
    send_tx(queue, sender);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}



















































































































































































































































































