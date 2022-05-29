use super::handler::Handler;
use super::Car;

pub struct Garage {
    handlers: Vec<Box<dyn Handler>>,
}

impl Garage {
    pub fn new() -> Garage {
        Garage {
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: impl Handler + 'static) {
        self.handlers.push(Box::new(handler));
    }

    pub fn handle_car(&self, car: &mut Car) {
        for handle in &self.handlers {
            handle.handle(car);
        }
    }
}

impl Default for Garage {
    fn default() -> Self {
        Self::new()
    }
}
