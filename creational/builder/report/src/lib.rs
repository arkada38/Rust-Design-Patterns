pub mod html_report_builder;
pub mod markdown_report_builder;

use std::fmt;

pub struct Report {
    content: String,
    items: i32,
    format: String
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\nItems: {}\nFormat: {}", self.content, self.items, self.format)
    }
}
