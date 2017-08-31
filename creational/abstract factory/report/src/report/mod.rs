pub mod markdown;
pub mod html;

pub trait Report {
    fn set_header(&mut self, header: &'static str);
    fn set_content(&mut self, content: &'static str);
}
