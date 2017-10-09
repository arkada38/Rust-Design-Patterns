use musician::Musician;

pub struct Guitarist {
    name: String
}

impl Guitarist {
    pub fn new(name: String) -> Guitarist {
        Guitarist { name }
    }

    pub fn play_cool_opening(&self) {
        self.output("starting with a steep entry");
    }

    pub fn play_cool_riffs(&self) {
        self.output("playing cool riffs");
    }

    pub fn play_another_cool_riffs(&self) {
        self.output("playing another cool riffs");
    }

    pub fn play_incredibly_cool_solo(&self) {
        self.output("playing incredibly cool solo");
    }

    pub fn play_final_accord(&self) {
        self.output("playing final accord");
    }
}

impl Musician for Guitarist {
    fn output(&self, text: &str) {
        println!("{} {}.", self.name, text);
    }
}
