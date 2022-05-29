pub mod transport {
    use super::delivery::Delivery;

    pub trait Transport {
        fn load(&self);
        fn set_delivery(&mut self, delivery: Box<dyn Delivery>);
        fn carry(&self);
        fn unload(&self);
    }

    pub struct Plane {
        delivery: Box<dyn Delivery>,
    }

    impl Plane {
        pub fn new(delivery: Box<dyn Delivery>) -> Plane {
            Plane { delivery }
        }
    }

    impl Transport for Plane {
        fn load(&self) {
            println!("Loading on the plane... Done!");
        }

        fn set_delivery(&mut self, delivery: Box<dyn Delivery>) {
            self.delivery = delivery;
        }

        fn carry(&self) {
            self.delivery.issue();
            println!("The plane flew off... Done!");
        }

        fn unload(&self) {
            println!("Unloading the plane... Done!\n");
        }
    }

    pub struct Train {
        delivery: Box<dyn Delivery>,
    }

    impl Train {
        pub fn new(delivery: Box<dyn Delivery>) -> Train {
            Train { delivery }
        }
    }

    impl Transport for Train {
        fn load(&self) {
            println!("Loading on the train... Done!");
        }

        fn set_delivery(&mut self, delivery: Box<dyn Delivery>) {
            self.delivery = delivery;
        }

        fn carry(&self) {
            self.delivery.issue();
            println!("The train left... Done!");
        }

        fn unload(&self) {
            println!("Unloading the train... Done!\n");
        }
    }
}

pub mod delivery {
    pub trait Delivery {
        fn issue(&self);
    }

    pub struct Express {}

    impl Delivery for Express {
        fn issue(&self) {
            println!("Super fast delivery for 3 days")
        }
    }

    pub struct Normal {}

    impl Delivery for Normal {
        fn issue(&self) {
            println!("Normal delivery for two weeks")
        }
    }
}
