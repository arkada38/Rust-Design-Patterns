use super::Musician;

pub struct Bassist {
    name: String,
}

impl Bassist {
    pub fn new<S: Into<String>>(name: S) -> Bassist {
        Bassist { name: name.into() }
    }

    pub fn follow_the_drums(&self) {
        self.output("follow the drums");
    }

    pub fn change_rhythm(&self, rhythm_type: &str) {
        self.output(&format!("switched to the rhythm of the {rhythm_type}"));
    }

    pub fn stop_playing(&self) {
        self.output("stop playing");
    }
}

impl Musician for Bassist {
    fn output(&self, text: &str) {
        println!("{} {text}.", self.name);
    }
}
