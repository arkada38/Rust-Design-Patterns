pub trait Policeman {
    fn set_next(&mut self, next: Box<dyn Policeman>);
    fn investigate(&self, crime: u8);
}

pub struct Detective {
    deduction: u8,
    next: Option<Box<dyn Policeman>>,
}

impl Detective {
    pub fn new(deduction: u8) -> Detective {
        Detective {
            deduction,
            next: None,
        }
    }
}

impl Policeman for Detective {
    fn set_next(&mut self, next: Box<dyn Policeman>) {
        self.next = Some(next);
    }

    fn investigate(&self, crime: u8) {
        if crime > self.deduction {
            println!("Detective: I can't investigate it. I need help.");

            match &self.next {
                &Some(ref next) => next.investigate(crime),
                &None => println!("Detective: Unimpossible for our department"),
            }
        } else {
            println!("Detective: I can do this.");
        }
    }
}

pub struct Patrolman {
    deduction: u8,
    next: Option<Box<dyn Policeman>>,
}

impl Patrolman {
    pub fn new(deduction: u8) -> Patrolman {
        Patrolman {
            deduction,
            next: None,
        }
    }
}

impl Policeman for Patrolman {
    fn set_next(&mut self, next: Box<dyn Policeman>) {
        self.next = Some(next);
    }

    fn investigate(&self, crime: u8) {
        if crime > self.deduction {
            println!("Patrolman: I'm just a patrolman. I need help.");

            match &self.next {
                &Some(ref next) => next.investigate(crime),
                &None => println!("Patrolman: Unimpossible for our department."),
            }
        } else {
            println!("Patrolman: It's easy. I can do this.");
        }
    }
}
