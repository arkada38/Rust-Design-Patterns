use std::fmt::{self, Display};

pub struct Config {
    width: i32,
    height: i32,
    title: String,
    is_minimized: bool,
}

pub struct ConfigBuilder {
    width: i32,
    height: i32,
    title: String,
    is_minimized: bool,
}

impl ConfigBuilder {
    pub fn new() -> ConfigBuilder {
        ConfigBuilder {
            width: 100,
            height: 50,
            title: "Builder".to_string(),
            is_minimized: false,
        }
    }

    pub fn with_width(mut self, width: i32) -> ConfigBuilder {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: i32) -> ConfigBuilder {
        self.height = height;
        self
    }

    pub fn with_title<S: Into<String> + Display>(mut self, title: S) -> ConfigBuilder {
        self.title = title.into();
        self
    }

    pub fn with_minimized(mut self, minimized: bool) -> ConfigBuilder {
        self.is_minimized = minimized;
        self
    }

    pub fn finalize(self) -> Config {
        Config {
            width: self.width,
            height: self.height,
            title: self.title,
            is_minimized: self.is_minimized,
        }
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Width: {}\nHeight: {}\nTitle: {}\nIs minimized: {}\n",
            self.width, self.height, self.title, self.is_minimized
        )
    }
}
