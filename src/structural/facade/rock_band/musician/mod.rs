pub mod bassist;
pub mod drummer;
pub mod guitarist;
pub mod vocalist;

pub trait Musician {
    fn output(&self, text: &str);
}
