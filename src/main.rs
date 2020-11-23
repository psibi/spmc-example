use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    let queue: Arc<Mutex<VecDeque<u32>>> = Arc::new(Mutex::new(VecDeque::new()));

    let producer_queue = Arc::clone(&queue);
    let producer = thread::spawn(move || loop {
        let mut producer_queue = producer_queue.lock().unwrap();
        producer_queue.push_back(rand::random());
        println!(
            "Producer threadID: {:?}, Queue Length: {}",
            thread::current().id(),
            producer_queue.len(),
        )
    });
    let mut consumer_handles = vec![];

    for _i in 0..10 {
        let consumer_queue = Arc::clone(&queue);
        let consumer = thread::spawn(move || loop {
            let mut consumer_queue = consumer_queue.lock().unwrap();
            let element = consumer_queue.pop_front();
            if element.is_some() {
                println!(
                    "Consumer threadID: {:?} {}",
                    thread::current().id(),
                    element.unwrap()
                );
            }
        });
        consumer_handles.push(consumer);
    }

    producer.join().unwrap();
    for handle in consumer_handles {
        handle.join().unwrap()
    }
}
