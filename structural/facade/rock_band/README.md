# Rock band

Sample project with *Facade*

## Structure

- Musician
  - Vocalist
  - Guitarist
  - Bassist
  - Drummer

- BlackSabbath

## Implementation

### Module musician

```rust
pub trait Musician {
    fn output(&self, text: &str);
}
```

### Module vocalist

```rust
pub struct Vocalist {
    name: String
}

impl Vocalist {
    pub fn new(name: String) -> Vocalist {
        Vocalist { name }
    }

    pub fn sing_couplet(&self, couplet_number: i32) {
        self.output(&format!("singing a couplet № {}", couplet_number));
    }

    pub fn sing_chorus(&self) {
        self.output("singing a chorus");
    }
}

impl Musician for Vocalist {
    fn output(&self, text: &str) {
        println!("{} {}.", self.name, text);
    }
}
```

### Module guitarist

```rust
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
```

### Module bassist

```rust
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
```

### Module drummer

```rust
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
```

### Module black_sabbath

```rust
pub struct BlackSabbath {
    vocalist: Vocalist,
    guitarist: Guitarist,
    bassist: Bassist,
    drummer: Drummer
}

impl BlackSabbath {
    pub fn new() -> BlackSabbath {
        BlackSabbath {
            vocalist: Vocalist::new("Ozzy Osbourne".to_string()),
            guitarist: Guitarist::new("Tony Iommi".to_string()),
            bassist: Bassist::new("Geezer Butler".to_string()),
            drummer: Drummer::new("Bill Ward".to_string())
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
```

### main

```rust
fn main() {
    let black_sabbath = BlackSabbath::new();
    black_sabbath.play_cool_song();
}
```

### output

```bash
Tony Iommi starting with a steep entry.
Bill Ward start playing.
Geezer Butler follow the drums.
Tony Iommi playing cool riffs.
Ozzy Osbourne singing a couplet № 1.
Geezer Butler switched to the rhythm of the chorus.
Tony Iommi playing another cool riffs.
Ozzy Osbourne singing a chorus.
Geezer Butler switched to the rhythm of the verse.
Tony Iommi playing cool riffs.
Ozzy Osbourne singing a couplet № 2.
Geezer Butler switched to the rhythm of the chorus.
Tony Iommi playing another cool riffs.
Ozzy Osbourne singing a chorus.
Geezer Butler switched to the rhythm of the verse.
Tony Iommi playing incredibly cool solo.
Tony Iommi playing cool riffs.
Ozzy Osbourne singing a couplet № 3.
Geezer Butler switched to the rhythm of the chorus.
Tony Iommi playing another cool riffs.
Ozzy Osbourne singing a chorus.
Geezer Butler switched to the rhythm of the verse.
Tony Iommi playing cool riffs.
Geezer Butler stop playing.
Bill Ward stop playing.
Tony Iommi playing final accord.
```
