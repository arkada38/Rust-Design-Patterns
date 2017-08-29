pub mod markdown;
pub mod html;

pub trait Report {
    fn set_header(&mut self, header: &'static str);
    fn set_content(&mut self, content: &'static str);

    fn print(&self);
}

pub fn print_report<T: Report>(mut report: T, header: &'static str, content: &'static str) {
    report.set_header(header);
    report.set_content(content);
    report.print();
}
