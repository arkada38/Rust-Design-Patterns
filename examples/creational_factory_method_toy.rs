use design_patterns::creational::factory_method::toy::{
    DollFactory, RobotFactory, Toy, ToyFactory,
};

fn main() {
    let doll = DollFactory.create("Alena");
    let robot = RobotFactory.create("Max");

    play(doll);
    play(robot);
}

fn play<T: Toy>(toy: T) {
    toy.sing();
    toy.say_name();
}
