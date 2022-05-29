mod drink;
mod launch_factory;
mod main_course;

pub use drink::{BusinessDrink, CheapDrink, Drink};
pub use launch_factory::{BusinessLaunchFactory, CheapLaunchFactory, LaunchFactory};
pub use main_course::{BusinessCourse, CheapCourse, MainCourse};

pub trait Launch<M, D>
where
    M: MainCourse,
    D: Drink,
{
    fn get_main_course(&self) -> M;
    fn get_drink(&self) -> D;

    fn print_prices<L>(&self, launch: &L)
    where
        M: MainCourse,
        D: Drink,
        L: Launch<M, D>,
    {
        let maine_course = launch.get_main_course();
        let drink = launch.get_drink();

        self.print_main_course_price(&maine_course);
        self.print_drink_price(&drink);
    }

    fn print_main_course_price(&self, main_course: &M) {
        println!("Main Course: {}$", main_course.get_price());
    }

    fn print_drink_price(&self, drink: &D) {
        println!("Drink: {}$\n", drink.get_price());
    }
}

pub struct CheapLaunch {
    pub main_course: CheapCourse,
    pub drink: CheapDrink,
}

impl CheapLaunch {
    pub fn new() -> CheapLaunch {
        CheapLaunch {
            main_course: CheapCourse::new(110),
            drink: CheapDrink::new(50),
        }
    }
}

impl Default for CheapLaunch {
    fn default() -> Self {
        Self::new()
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
    pub drink: BusinessDrink,
}

impl BusinessLaunch {
    pub fn new() -> BusinessLaunch {
        BusinessLaunch {
            main_course: BusinessCourse::new(1_500),
            drink: BusinessDrink::new(250),
        }
    }
}

impl Default for BusinessLaunch {
    fn default() -> Self {
        Self::new()
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
