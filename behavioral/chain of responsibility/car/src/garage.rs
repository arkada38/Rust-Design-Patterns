use super::Car;
use super::handler::Handler;

pub struct Garage<'a> {
    handlers: Vec<Box<Handler +'a>>,
}

impl<'a> Garage<'a> {
    pub fn new() -> Garage<'a> {
        Garage{handlers: Vec::new()}
    }

    pub fn add_handler<T>(&mut self, handler: T)
        where T: 'a + Handler {
        self.handlers.push(Box::new(handler));
    }

    pub fn handle_car(&self, car: &mut Car) {
        for handle in &self.handlers {
            handle.handle(car);
        }
    }
}
