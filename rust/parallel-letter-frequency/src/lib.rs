use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver};

pub fn frequency(text: &[&str], workers: usize) -> HashMap<char, u32> {
    let (collector_tx, collector_rx) = mpsc::channel();
    let mut children = vec![];

    // start workers
    for _ in 0..workers {
        let (tx, rx): (Sender<Option<String>>, Receiver<Option<String>>) = mpsc::channel();
        let collector = collector_tx.clone();
        let handle = thread::spawn(move || {
            let mut freq = HashMap::<char, u32>::new();
            loop {
                if let Ok(msg) = rx.recv() {
                    match msg {
                        Some(text) => {
                            for c in text.chars().filter(|c| c.is_alphabetic()) {
                                *freq.entry(c).or_insert(0) += 1;
                            }
                        }
                        None => {
                            collector.send(freq).unwrap();
                            break;
                        }
                    }
                }
            }
        });
        children.push((handle, tx));
    }

    // send work
    for (i, &piece) in text.iter().enumerate() {
        let (_, ref tx) = children[i % workers];
        tx.send(Some(piece.to_owned())).unwrap();
    }

    // tell all workers to finish up
    for (handle, tx) in children {
        tx.send(None).unwrap();
        handle.join().unwrap();
    }

    drop(collector_tx);
    let mut ret = HashMap::new();
    while let Ok(msg) = collector_rx.recv() {
        for (character, count) in msg {
            let character = character.to_lowercase().next().unwrap();
            *ret.entry(character).or_insert(0) += count;
        }
    }
    ret
}
