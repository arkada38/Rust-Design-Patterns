use musician::Musician;

pub struct Bassist {
    name: String
}

impl Bassist {
    pub fn new(name: String) -> Bassist {
        Bassist { name }
    }

    pub fn follow_the_drums(&self) {
        self.output("follow the drums");
    }

    pub fn change_rhythm(&self, rhythm_type: &str) {
        self.output(&format!("switched to the rhythm of the {}", rhythm_type));
    }

    pub fn stop_playing(&self) {
        self.output("stop playing");
    }
}

impl Musician for Bassist {
    fn output(&self, text: &str) {
        println!("{} {}.", self.name, text);
    }
}
