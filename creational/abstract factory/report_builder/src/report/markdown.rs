use report::Report;

pub struct MarkdownReport {
    header: &'static str,
    content: &'static str,
    format: &'static str
}

impl MarkdownReport {
    pub fn new() -> MarkdownReport {
        MarkdownReport{
            header: "",
            content: "",
            format: "md"
        }
    }
}

impl Report for MarkdownReport {
    fn set_header(&mut self, header: &'static str) {
        self.header = header;
    }

    fn get_header(&self) -> &'static str {
        self.header
    }

    fn set_content(&mut self, content: &'static str) {
        self.content = content;
    }

    fn get_content(&self) -> &'static str {
        self.content
    }

    fn get_format(&self) -> &'static str {
        self.format
    }

    fn print(&self) {
        println!("# {}  ", self.header);
        println!("{}  ", self.content);
        println!("Format: {}  ", self.format);
    }
}