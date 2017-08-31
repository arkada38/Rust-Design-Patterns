extern crate launch;

use launch::launch::Launch;
use launch::launch::main_course::MainCourse;
use launch::launch::drink::Drink;
use launch::launch_factory::{ LaunchFactory, CheapLaunchFactory, BusinessLaunchFactory };

fn main() {
    let cheap_launch = CheapLaunchFactory.create();
    let cheap_main_course = cheap_launch.get_main_course();
    let cheap_drink = cheap_launch.get_drink();

    print_main_course_price(cheap_main_course);
    print_drink_price(cheap_drink);

    let business_launch = BusinessLaunchFactory.create();
    let business_main_course = business_launch.get_main_course();
    let business_drink = business_launch.get_drink();

    print_main_course_price(business_main_course);
    print_drink_price(business_drink);
}

fn print_main_course_price<M: MainCourse>(main_course: M) {
    println!("Main Course: {}$", main_course.get_price());
}

fn print_drink_price<D: Drink>(drink: D) {
    println!("Drink: {}$", drink.get_price());
}
