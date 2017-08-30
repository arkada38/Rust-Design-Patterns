extern crate report_builder;

use report_builder::Report;
use report_builder::html_report_builder::HtmlReportBuilder;
use report_builder::markdown_report_builder::MarkdownReportBuilder;

fn main() {
    let html_report: Report = HtmlReportBuilder::new()
        .with_header("First header")
        .with_paragraph("First paragraph")
        .with_header("Second header")
        .with_paragraph("Second paragraph")
        .finish();

    println!("{}", html_report);


    let markdown_report: Report = MarkdownReportBuilder::new()
        .with_header("First header")
        .with_paragraph("First paragraph")
        .with_header("Second header")
        .with_paragraph("Second paragraph")
        .finish();

    println!("{}", markdown_report);
}
