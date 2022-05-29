use super::Report;
use std::fmt;

pub struct MarkdownReport {
    header: String,
    content: String,
}

impl MarkdownReport {
    pub fn new() -> MarkdownReport {
        MarkdownReport {
            header: String::new(),
            content: String::new(),
        }
    }
}

impl Default for MarkdownReport {
    fn default() -> Self {
        Self::new()
    }
}

impl Report for MarkdownReport {
    fn set_header<S: Into<String>>(&mut self, header: S) {
        self.header = header.into();
    }
    fn set_content<S: Into<String>>(&mut self, content: S) {
        self.content = content.into();
    }
}

impl fmt::Display for MarkdownReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "# {}\n\n{}\n", self.header, self.content)
    }
}
