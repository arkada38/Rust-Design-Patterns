use design_patterns::behavioral::chain_of_responsibility::car::{
    garage::Garage,
    handler::{HandleFuel, HandleKm, HandleOil},
    Car,
};

fn main() {
    let mut car = Car::new(11100, 1, 3);

    let mut garage = Garage::new();

    garage.add_handler(HandleFuel {});
    garage.add_handler(HandleOil {});
    garage.add_handler(HandleKm {});

    garage.handle_car(&mut car);
}
