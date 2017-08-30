extern crate report;

use report::report::print_report;
use report::report_factory::{MarkdownReportFactory, HtmlReportFactory, ReportFactory};

fn main() {
    let markdown_factory = MarkdownReportFactory;
    let html_factory = HtmlReportFactory;

    let markdown_report = markdown_factory.new_report();
    let html_report = html_factory.new_report();

    print_report(
        markdown_report,
        "Markdown header",
        "Markdown content"
    );

    print_report(
        html_report,
        "Html header",
        "Html content"
    );
}
