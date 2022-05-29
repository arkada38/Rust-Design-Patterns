use super::musician::bassist::Bassist;
use super::musician::drummer::Drummer;
use super::musician::guitarist::Guitarist;
use super::musician::vocalist::Vocalist;

pub struct BlackSabbath {
    vocalist: Vocalist,
    guitarist: Guitarist,
    bassist: Bassist,
    drummer: Drummer,
}

impl BlackSabbath {
    pub fn new() -> BlackSabbath {
        BlackSabbath {
            vocalist: Vocalist::new("Ozzy Osbourne"),
            guitarist: Guitarist::new("Tony Iommi"),
            bassist: Bassist::new("Geezer Butler"),
            drummer: Drummer::new("Bill Ward"),
        }
    }

    pub fn play_cool_song(&self) {
        self.guitarist.play_cool_opening();
        self.drummer.start_playing();
        self.bassist.follow_the_drums();
        self.guitarist.play_cool_riffs();
        self.vocalist.sing_couplet(1);
        self.bassist.change_rhythm("chorus");
        self.guitarist.play_another_cool_riffs();
        self.vocalist.sing_chorus();
        self.bassist.change_rhythm("verse");
        self.guitarist.play_cool_riffs();
        self.vocalist.sing_couplet(2);
        self.bassist.change_rhythm("chorus");
        self.guitarist.play_another_cool_riffs();
        self.vocalist.sing_chorus();
        self.bassist.change_rhythm("verse");
        self.guitarist.play_incredibly_cool_solo();
        self.guitarist.play_cool_riffs();
        self.vocalist.sing_couplet(3);
        self.bassist.change_rhythm("chorus");
        self.guitarist.play_another_cool_riffs();
        self.vocalist.sing_chorus();
        self.bassist.change_rhythm("verse");
        self.guitarist.play_cool_riffs();
        self.bassist.stop_playing();
        self.drummer.stop_playing();
        self.guitarist.play_final_accord();
    }
}

impl Default for BlackSabbath {
    fn default() -> Self {
        Self::new()
    }
}
