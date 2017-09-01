pub mod html_report_builder;
pub mod markdown_report_builder;

use report::Report;

pub trait ReportBuilder<T: ReportBuilder<T>> {
    fn with_header(self, header: &'static str) -> T;
    fn with_paragraph(self, paragraph: &'static str) -> T;
    fn finish(self) -> Report;
}
