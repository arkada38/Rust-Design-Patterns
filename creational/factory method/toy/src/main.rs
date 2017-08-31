extern crate toy;

use toy::toy::Toy;
use toy::toy_factory::{ ToyFactory, DollFactory, RobotFactory };

fn main() {
    let doll = DollFactory.create("Alena".to_string());
    let robot = RobotFactory.create("Max".to_string());

    play(doll);
    play(robot);
}

fn play<T: Toy>(toy: T) {
    toy.sing();
    toy.say_name();
}
