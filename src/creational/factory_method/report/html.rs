use super::Report;
use std::fmt;

pub struct HtmlReport {
    header: String,
    content: String,
}

impl HtmlReport {
    pub fn new() -> HtmlReport {
        HtmlReport {
            header: String::new(),
            content: String::new(),
        }
    }
}

impl Default for HtmlReport {
    fn default() -> Self {
        Self::new()
    }
}

impl Report for HtmlReport {
    fn set_header<S: Into<String>>(&mut self, header: S) {
        self.header = header.into();
    }
    fn set_content<S: Into<String>>(&mut self, content: S) {
        self.content = content.into();
    }
}

impl fmt::Display for HtmlReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<h1>{}</h1>\n<p>{}</p>\n", self.header, self.content)
    }
}
