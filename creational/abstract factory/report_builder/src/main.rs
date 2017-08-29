extern crate report_builder;

use report_builder::report::Report;
use report_builder::report_factory::{MarkdownReportFactory, HtmlReportFactory, ReportFactory};

fn main() {
    let markdown_factory = MarkdownReportFactory;
    let html_factory = HtmlReportFactory;

    let mut markdown = markdown_factory.create_report();
    let mut html = html_factory.create_report();

    markdown.set_header("Markdown header");
    markdown.set_content("Markdown content");
    markdown.print();

    html.set_header("Html header");
    html.set_content("Html content");
    html.print();
}
