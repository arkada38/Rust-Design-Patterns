pub trait Policeman {
    fn set_next(&mut self, next: Box<dyn Policeman>);
    fn investigate(&self, crime: u8);
}

pub struct Detective {
    deduction: u8,
    next: Option<Box<dyn Policeman>>,
    name: String,
}

impl Detective {
    pub fn new<S: Into<String>>(deduction: u8, name: S) -> Detective {
        Detective {
            deduction,
            next: None,
            name: name.into(),
        }
    }
}

impl Policeman for Detective {
    fn set_next(&mut self, next: Box<dyn Policeman>) {
        self.next = Some(next);
    }

    fn investigate(&self, crime: u8) {
        if crime > self.deduction {
            println!(
                "Detective {}: I can't investigate it. I need help.",
                self.name
            );

            match self.next {
                Some(ref next) => next.investigate(crime),
                None => println!("Detective {}: Unimpossible for our department", self.name),
            }
        } else {
            println!("Detective {}: I can do this.", self.name);
        }
    }
}

pub struct Patrolman {
    deduction: u8,
    next: Option<Box<dyn Policeman>>,
    name: String,
}

impl Patrolman {
    pub fn new<S: Into<String>>(deduction: u8, name: S) -> Patrolman {
        Patrolman {
            deduction,
            next: None,
            name: name.into(),
        }
    }
}

impl Policeman for Patrolman {
    fn set_next(&mut self, next: Box<dyn Policeman>) {
        self.next = Some(next);
    }

    fn investigate(&self, crime: u8) {
        if crime > self.deduction {
            println!(
                "Patrolman {}: I'm just a patrolman. I need help.",
                self.name
            );

            match self.next {
                Some(ref next) => next.investigate(crime),
                None => println!("Patrolman {}: Unimpossible for our department.", self.name),
            }
        } else {
            println!("Patrolman {}: It's easy. I can do this.", self.name);
        }
    }
}
