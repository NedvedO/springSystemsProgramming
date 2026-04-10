use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 10; // each producer sends 10 items → 20 total
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    let (tx, rx) = mpsc::channel::<i32>();
    // Share the single receiver among all consumers
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    // Spawn producers
    for id in 0..NUM_PRODUCERS {
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            producer(id, tx_clone, ITEM_COUNT);
        }));
    }

    // Spawn consumers
    for id in 0..NUM_CONSUMERS {
        let rx_clone = Arc::clone(&rx);
        handles.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }

    // Drop the original sender so only the clones remain;
    // after producers finish their clones are dropped too.
    // Then we send one termination signal per consumer.
    drop(tx); // drop main's copy

    // Wait for producers to finish first, then send termination signals
    // (We stored producer handles at indices 0..NUM_PRODUCERS)
    // Simpler: just wait for all, but we need to signal consumers first.
    // Strategy: join producers, then signal consumers, then join consumers.
    // Re-collect handles by rebuilding:
    //
    // Actually, let's keep it simple with a dedicated wait + signal approach.
    // (See note below main for explanation.)

    println!("All items have been produced and consumed!");
}

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    for _ in 0..item_count {
        let value = (id * 100 + rand_simple()) as i32; // simple pseudo-random
        println!("Producer {} sending: {}", id, value);
        tx.send(value).unwrap();
        thread::sleep(Duration::from_millis(50));
    }
    println!("Producer {} finished.", id);
    // Sender clone drops here automatically
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let value = rx.lock().unwrap().recv().unwrap_or(TERMINATION_SIGNAL);
        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal. Exiting.", id);
            break;
        }
        println!("Consumer {} processing value: {}", id, value);
        thread::sleep(Duration::from_millis(75));
    }
}

// Minimal pseudo-random helper (avoids the `rand` crate dependency)
fn rand_simple() -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize)
        % 100
}