use design_patterns::creational::singleton;
use std::{thread, time::Duration};

/// Example with *Singleton* pattern
fn main() {
    // Let's use the singleton in a few threads
    let threads = (0..5)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(i * 10));
                let s = singleton::singleton();
                let mut data = s.inner.lock().unwrap();
                data.set_area(i * 15);
                data.set_population(12 + i);
            })
        })
        .collect::<Vec<_>>();

    // And let's check the singleton every so often
    for i in 0..10 {
        thread::sleep(Duration::from_millis(5));

        let s = singleton::singleton();
        let data = s.inner.lock().unwrap();
        println!("{} {:?}", i, *data);
    }

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }
}
