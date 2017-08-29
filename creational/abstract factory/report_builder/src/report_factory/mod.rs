use report::Report;
use report::markdown::MarkdownReport;
use report::html::HtmlReport;

pub trait ReportFactory<R: Report> {
    fn create_report(&self) -> R;
}

pub struct MarkdownReportFactory;
pub struct HtmlReportFactory;

impl ReportFactory<MarkdownReport> for MarkdownReportFactory {
    fn create_report(&self) -> MarkdownReport {
        MarkdownReport::new()
    }
}

impl ReportFactory<HtmlReport> for HtmlReportFactory {
    fn create_report(&self) -> HtmlReport {
        HtmlReport::new()
    }
}
