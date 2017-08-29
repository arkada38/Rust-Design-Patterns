pub mod markdown;
pub mod html;

pub trait Report {
    fn set_header(&mut self, header: &'static str);
    fn get_header(&self) -> &'static str;

    fn set_content(&mut self, content: &'static str);
    fn get_content(&self) -> &'static str;

    fn get_format(&self) -> &'static str;

    fn print(&self);
}
