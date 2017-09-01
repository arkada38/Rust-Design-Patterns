use report::Report;
use report_builder::ReportBuilder;

pub struct MarkdownReportBuilder {
    content: String,
    items: i32
}

impl MarkdownReportBuilder {
    pub fn new() -> MarkdownReportBuilder {
        MarkdownReportBuilder {
            content: "".to_string(),
            items: 0
        }
    }
}

impl ReportBuilder<MarkdownReportBuilder> for MarkdownReportBuilder {
    fn with_header(mut self, header: &'static str) -> MarkdownReportBuilder {
        self.content.push_str(&format!("# {}\n\n", header));
        self.items += 1;
        self
    }

    fn with_paragraph(mut self, paragraph: &'static str) -> MarkdownReportBuilder {
        self.content.push_str(&format!("{}\n\n", paragraph));
        self.items += 1;
        self
    }

    fn finish(self) -> Report {
        Report {
            content: self.content,
            items: self.items,
            format: "Markdown\n".to_string()
        }
    }
}
