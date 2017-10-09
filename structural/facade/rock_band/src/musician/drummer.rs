use musician::Musician;

pub struct Drummer {
    name: String
}

impl Drummer {
    pub fn new(name: String) -> Drummer {
        Drummer { name }
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
        println!("{} {}.", self.name, text);
    }
}
