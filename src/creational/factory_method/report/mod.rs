mod html;
mod markdown;

pub use html::HtmlReport;
pub use markdown::MarkdownReport;

pub trait Report {
    fn set_header<S: Into<String>>(&mut self, header: S);
    fn set_content<S: Into<String>>(&mut self, content: S);
}

pub trait ReportFactory<R: Report> {
    fn new_report(&self) -> R;
}

pub struct MarkdownReportFactory;
pub struct HtmlReportFactory;

impl ReportFactory<MarkdownReport> for MarkdownReportFactory {
    fn new_report(&self) -> MarkdownReport {
        MarkdownReport::new()
    }
}

impl ReportFactory<HtmlReport> for HtmlReportFactory {
    fn new_report(&self) -> HtmlReport {
        HtmlReport::new()
    }
}
