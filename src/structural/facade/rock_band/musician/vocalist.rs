use super::Musician;

pub struct Vocalist {
    name: String,
}

impl Vocalist {
    pub fn new<S: Into<String>>(name: S) -> Vocalist {
        Vocalist { name: name.into() }
    }

    pub fn sing_couplet(&self, couplet_number: i32) {
        self.output(&format!("singing a couplet № {couplet_number}"));
    }

    pub fn sing_chorus(&self) {
        self.output("singing a chorus");
    }
}

impl Musician for Vocalist {
    fn output(&self, text: &str) {
        println!("{} {text}.", self.name);
    }
}
