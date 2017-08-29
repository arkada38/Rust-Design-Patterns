use report::Report;

pub struct HtmlReport {
    header: &'static str,
    content: &'static str,
    format: &'static str
}

impl HtmlReport {
    pub fn new() -> HtmlReport {
        HtmlReport{
            header: "",
            content: "",
            format: "html"
        }
    }
}

impl Report for HtmlReport {
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
        println!("<h1>{}</h1>", self.header);
        println!("<p>{}</p>", self.content);
        println!("<p>Format: {}</p>", self.format);
    }
}