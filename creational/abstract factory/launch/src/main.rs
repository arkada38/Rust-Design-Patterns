extern crate launch;

use launch::launch::Launch;
use launch::launch::main_course::MainCourse;
use launch::launch::drink::Drink;
use launch::launch_factory::{ LaunchFactory, CheapLaunchFactory, BusinessLaunchFactory };

fn main() {
    let cheap_launch = CheapLaunchFactory.create();
    print_prices(cheap_launch);

    let business_launch = BusinessLaunchFactory.create();
    print_prices(business_launch);
}

fn print_prices<M, D, L>(launch: L)
    where M: MainCourse,
          D: Drink,
          L: Launch<M, D> {
    let maine_course = launch.get_main_course();
    let drink = launch.get_drink();

    print_main_course_price(maine_course);
    print_drink_price(drink);
}

fn print_main_course_price<M: MainCourse>(main_course: M) {
    println!("Main Course: {}$", main_course.get_price());
}

fn print_drink_price<D: Drink>(drink: D) {
    println!("Drink: {}$\n", drink.get_price());
}
