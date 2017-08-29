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

    fn print(&self) {
        println!("<h1>{}</h1>", self.header);
        println!("<p>{}</p>", self.content);
    }
}
