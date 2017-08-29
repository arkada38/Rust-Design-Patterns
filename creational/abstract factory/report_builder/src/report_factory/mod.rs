use report::Report;
use report::markdown::MarkdownReport;
use report::html::HtmlReport;

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
