use std::fmt;
use report::Report;

pub struct HtmlReport {
    header: &'static str,
    content: &'static str
}

impl HtmlReport {
    pub fn new() -> HtmlReport {
        HtmlReport{
            header: "",
            content: ""
        }
    }
}

impl Report for HtmlReport {
    fn set_header(&mut self, header: &'static str) {
        self.header = header;
    }
    fn set_content(&mut self, content: &'static str) {
        self.content = content;
    }
}

impl fmt::Display for HtmlReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<h1>{}</h1>\n<p>{}</p>\n", self.header, self.content)
    }
}
