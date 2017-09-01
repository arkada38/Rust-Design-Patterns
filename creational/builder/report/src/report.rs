use std::fmt;

pub struct Report {
    pub content: String,
    pub items: i32,
    pub format: String
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nItems: {}\nFormat: {}", self.content, self.items, self.format)
    }
}
