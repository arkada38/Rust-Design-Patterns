use std::fmt::{self, Display};

pub struct Report {
    pub content: String,
    pub items: i32,
    pub format: String,
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\nItems: {}\nFormat: {}",
            self.content, self.items, self.format
        )
    }
}

pub trait ReportBuilder<T: ReportBuilder<T>> {
    fn with_header<S: Into<String> + Display>(self, header: S) -> T;
    fn with_paragraph<S: Into<String> + Display>(self, paragraph: S) -> T;
    fn finish(self) -> Report;
}

pub struct HtmlReportBuilder {
    content: String,
    items: i32,
}

impl HtmlReportBuilder {
    pub fn new() -> HtmlReportBuilder {
        HtmlReportBuilder {
            content: "".to_string(),
            items: 0,
        }
    }
}

impl Default for HtmlReportBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportBuilder<HtmlReportBuilder> for HtmlReportBuilder {
    fn with_header<S: Into<String> + Display>(mut self, header: S) -> HtmlReportBuilder {
        self.content.push_str(&format!("<h1>{}</h1>\n", header));
        self.items += 1;
        self
    }

    fn with_paragraph<S: Into<String> + Display>(mut self, paragraph: S) -> HtmlReportBuilder {
        self.content.push_str(&format!("<p>{}</p>\n", paragraph));
        self.items += 1;
        self
    }

    fn finish(self) -> Report {
        Report {
            content: self.content,
            items: self.items,
            format: "Html\n".to_string(),
        }
    }
}

pub struct MarkdownReportBuilder {
    content: String,
    items: i32,
}

impl MarkdownReportBuilder {
    pub fn new() -> MarkdownReportBuilder {
        MarkdownReportBuilder {
            content: "".to_string(),
            items: 0,
        }
    }
}

impl Default for MarkdownReportBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ReportBuilder<MarkdownReportBuilder> for MarkdownReportBuilder {
    fn with_header<S: Into<String> + Display>(mut self, header: S) -> MarkdownReportBuilder {
        self.content.push_str(&format!("# {}\n\n", header));
        self.items += 1;
        self
    }

    fn with_paragraph<S: Into<String> + Display>(mut self, paragraph: S) -> MarkdownReportBuilder {
        self.content.push_str(&format!("{}\n\n", paragraph));
        self.items += 1;
        self
    }

    fn finish(self) -> Report {
        Report {
            content: self.content,
            items: self.items,
            format: "Markdown\n".to_string(),
        }
    }
}
