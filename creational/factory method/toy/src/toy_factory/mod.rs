use toy::{ Doll, Robot };

pub trait ToyFactory<T> {
    fn create(&self, String) -> T;
}

pub struct DollFactory;
pub struct RobotFactory;

impl ToyFactory<Doll> for DollFactory {
    fn create(&self, name: String) -> Doll {
        Doll { name }
    }
}

impl ToyFactory<Robot> for RobotFactory {
    fn create(&self, name: String) -> Robot {
        Robot { name }
    }
}
