use std::fmt::Display;

use design_patterns::creational::factory_method::report::{
    HtmlReportFactory, MarkdownReportFactory, Report, ReportFactory,
};

fn main() {
    let markdown_factory = MarkdownReportFactory;
    let html_factory = HtmlReportFactory;

    let markdown_report = markdown_factory.new_report();
    let html_report = html_factory.new_report();

    let header = "The Unfaithful Friend";
    let content = "There was a large berry tree on the bank of a river. \
    On this tree lived a monkey called Rhesa. He was a clever and good hearted monkey...";

    print_report(markdown_report, header, content);

    print_report(html_report, header, content);
}

fn print_report<T: Report + Display>(mut report: T, header: &'static str, content: &'static str) {
    report.set_header(header);
    report.set_content(content);

    println!("{}", report);
}
