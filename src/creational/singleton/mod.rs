//! # Singleton
//!
//! ## Type
//!
//! Creational
//!
//! ## Description
//!
//! Ensure a struct only has one instance and provide a global point of access to it.

use std::mem;
use std::sync::{Arc, Mutex, Once};

#[derive(Clone)]
pub struct SingletonReader {
    // Since we will be used in many threads, we need to protect concurrent access
    pub inner: Arc<Mutex<World>>,
}

pub fn singleton() -> SingletonReader {
    // Initialize it to a null value
    static mut SINGLETON: *const SingletonReader = 0 as *const SingletonReader;
    static ONCE: Once = Once::new();

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = SingletonReader {
                inner: Arc::new(Mutex::new(World::new(0, 0))),
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });

        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

#[derive(Debug)]
pub struct World {
    area: u64,
    population: u64,
}

impl World {
    pub fn new(area: u64, population: u64) -> Self {
        Self { area, population }
    }

    pub fn set_area(&mut self, area: u64) {
        self.area = area;
    }

    pub fn set_population(&mut self, population: u64) {
        self.population = population;
    }
}
