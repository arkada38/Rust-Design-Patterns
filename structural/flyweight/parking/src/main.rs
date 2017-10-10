extern crate parking;

use parking::Parking;
use parking::car::car_type::{Body, Colour};

fn main() {
    let mut parking = Parking::new();
    parking.add_car("NFS MW", 13, Body::Coupe, Colour::Red);
    parking.add_car("MW2017", 15, Body::Truck, Colour::Yellow);
    parking.add_car("BIG11", 37, Body::Sedan, Colour::Black);
    parking.add_car("KING", 64, Body::Coupe, Colour::Red);
    parking.add_car("MAN", 73, Body::Coupe, Colour::Red);
    parking.add_car("TE64G", 18, Body::Truck, Colour::Yellow);
    parking.add_car("SMILE", 39, Body::Sedan, Colour::Black);
    parking.add_car("DARK01", 24, Body::Truck, Colour::Black);
    parking.add_car("DARK03", 25, Body::Truck, Colour::Black);
    parking.add_car("DARK05", 26, Body::Truck, Colour::Black);

    parking.print();
}
