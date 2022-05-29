use design_patterns::structural::flyweight::parking::{
    car::car_type::{Body, Colour},
    Parking,
};

fn main() {
    use Body::{Coupe, Sedan, Truck};
    use Colour::{Black, Red, Yellow};

    let mut parking = Parking::new();
    parking.add_car("NFS MW", 13, Coupe, Red);
    parking.add_car("MW2017", 15, Truck, Yellow);
    parking.add_car("BIG11", 37, Sedan, Black);
    parking.add_car("KING", 64, Coupe, Red);
    parking.add_car("MAN", 73, Coupe, Red);
    parking.add_car("TE64G", 18, Truck, Yellow);
    parking.add_car("SMILE", 39, Sedan, Black);
    parking.add_car("DARK01", 24, Truck, Black);
    parking.add_car("DARK03", 25, Truck, Black);
    parking.add_car("DARK05", 26, Truck, Black);

    parking.print();
}
