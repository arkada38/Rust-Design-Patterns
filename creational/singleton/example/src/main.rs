use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::time::Duration;
use std::{mem, thread};

#[derive(Clone)]
struct SingletonReader {
    // Since we will be used in many threads, we need to protect concurrent access
    inner: Arc<Mutex<World>>
}

fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = ONCE_INIT;

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                inner: Arc::new(Mutex::new(World { area: 0, population: 0 }))
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

fn main() {
    // Let's use the singleton in a few threads
    let threads: Vec<_> = (0..5).map(|i| {
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 10));
            let s = singleton();
            let mut data = s.inner.lock().unwrap();
            // *data = World { area: i * 15, population: 12 + i };
            data.set_area(i * 15);
            data.set_population(12 + i);
        })
    }).collect();

    // And let's check the singleton every so often
    for i in 0u64..10 {
        thread::sleep(Duration::from_millis(5));

        let s = singleton();
        let data = s.inner.lock().unwrap();
        println!("{} {:?}", i, *data);
    }

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }
}

#[derive(Debug)]
struct World {
    area: u64,
    population: u64
}

impl World {
    fn set_area(&mut self, area: u64) {
        self.area = area;
    }

    fn set_population(&mut self, population: u64) {
        self.population = population;
    }
}
