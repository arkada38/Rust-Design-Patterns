use std::fmt;
use report::Report;

pub struct MarkdownReport {
    header: &'static str,
    content: &'static str
}

impl MarkdownReport {
    pub fn new() -> MarkdownReport {
        MarkdownReport{
            header: "",
            content: ""
        }
    }
}

impl Report for MarkdownReport {
    fn set_header(&mut self, header: &'static str) {
        self.header = header;
    }
    fn set_content(&mut self, content: &'static str) {
        self.content = content;
    }
}

impl fmt::Display for MarkdownReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "# {}  \n{}  \n", self.header, self.content)
    }
}
