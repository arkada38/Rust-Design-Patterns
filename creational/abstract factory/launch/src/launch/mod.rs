pub mod main_course;
pub mod drink;

use launch::main_course::{ MainCourse, CheapCourse, BusinessCourse };
use launch::drink::{ Drink, CheapDrink, BusinessDrink };

pub trait Launch<M: MainCourse, D: Drink> {
    fn get_main_course(&self) -> M;
    fn get_drink(&self) -> D;
}

pub struct CheapLaunch {
    pub main_course: CheapCourse,
    pub drink: CheapDrink
}

impl CheapLaunch {
    pub fn new() -> CheapLaunch {
        CheapLaunch {
            main_course: CheapCourse::new(110),
            drink: CheapDrink::new(50)
        }
    }
}

impl Launch<CheapCourse, CheapDrink> for CheapLaunch {
    fn get_main_course(&self) -> CheapCourse {
        self.main_course
    }

    fn get_drink(&self) -> CheapDrink {
        self.drink
    }
}

pub struct BusinessLaunch {
    pub main_course: BusinessCourse,
    pub drink: BusinessDrink
}

impl BusinessLaunch {
    pub fn new() -> BusinessLaunch {
        BusinessLaunch {
            main_course: BusinessCourse::new(1500),
            drink: BusinessDrink::new(250)
        }
    }
}

impl Launch<BusinessCourse, BusinessDrink> for BusinessLaunch {
    fn get_main_course(&self) -> BusinessCourse {
        self.main_course
    }

    fn get_drink(&self) -> BusinessDrink {
        self.drink
    }
}
