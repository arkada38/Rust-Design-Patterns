# Police department

Sample project with *Chain of Responsibility*

Let's create a chain of policemen (Jack -> Tom -> Chuck). Every officer has an individual level of deduction.
And every crime has a difficult lvl. Officer can investigate the crime by himself if his deduction is not less than crime's difficulty lvl.
The officer passes the crime's case if the crime's difficulty lvl bigger than the officer's deduction.

## Structure

- Policeman
  - Detective
  - Patrolman

## Implementation

### Policeman trait

```rust
pub trait Policeman {
    fn set_next(&mut self, next: Box<Policeman>);
    fn investigate(&self, crime: u8);
}
```

### Detective struct

```rust
pub struct Detective {
    deduction: u8,
    next: Option<Box<Policeman>>,
}

impl Detective {
    pub fn new(deduction: u8) -> Detective {
        Detective {deduction, next: None}
    }
}

impl Policeman for Detective {
    fn set_next(&mut self, next: Box<Policeman>) {
        self.next = Some(next);
    }

    fn investigate(&self, crime: u8) {
        if crime > self.deduction {
            println!("Detective: I can't investigate it. I need help.");

            match &self.next {
                &Some(ref next) => next.investigate(crime),
                &None => println!("Detective: Unimpossible for our department")
            }
        } else {
            println!("Detective: I can do this.");
        }
    }
}
```

### Patrolman struct

```rust
pub struct Patrolman {
    deduction: u8,
    next: Option<Box<Policeman>>,
}

impl Patrolman {
    pub fn new(deduction: u8) -> Patrolman {
        Patrolman {deduction, next: None}
    }
}

impl Policeman for Patrolman {
    fn set_next(&mut self, next: Box<Policeman>) {
        self.next = Some(next);
    }

    fn investigate(&self, crime: u8) {
        if crime > self.deduction {
            println!("Patrolman: I'm just a patrolman. I need help.");

            match &self.next {
                &Some(ref next) => next.investigate(crime),
                &None => println!("Patrolman: Unimpossible for our department.")
            }
        } else {
            println!("Patrolman: It's easy. I can do this.");
        }
    }
}
```

### main

```rust
fn main() {
    let chuck = Detective::new(8);

    let mut tom = Detective::new(5);
    tom.set_next(Box::new(chuck));

    let mut jack = Patrolman::new(3);
    jack.set_next(Box::new(tom));

    jack.investigate(3);
    jack.investigate(4);
    jack.investigate(6);
    jack.investigate(9);
}
```

### output

```bash
Patrolman: It's easy. I can do this.
Patrolman: I'm just a patrolman. I need help.
Detective: I can do this.
Patrolman: I'm just a patrolman. I need help.
Detective: I can't investigate it. I need help.
Detective: I can do this.
Patrolman: I'm just a patrolman. I need help.
Detective: I can't investigate it. I need help.
Detective: I can't investigate it. I need help.
Detective: Unimpossible for our department
```
