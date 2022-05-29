use super::Musician;

pub struct Drummer {
    name: String,
}

impl Drummer {
    pub fn new<S: Into<String>>(name: S) -> Drummer {
        Drummer { name: name.into() }
    }

    pub fn start_playing(&self) {
        self.output("start playing");
    }

    pub fn stop_playing(&self) {
        self.output("stop playing");
    }
}

impl Musician for Drummer {
    fn output(&self, text: &str) {
        println!("{} {text}.", self.name);
    }
}
