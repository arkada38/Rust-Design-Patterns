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

    fn print(&self) {
        println!("# {}  ", self.header);
        println!("{}  ", self.content);
    }
}
