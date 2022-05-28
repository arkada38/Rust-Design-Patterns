use design_patterns::creational::abstract_factory::launch::{
    BusinessLaunchFactory, CheapLaunchFactory, Launch, LaunchFactory,
};

fn main() {
    let cheap_launch = CheapLaunchFactory.create();
    cheap_launch.print_prices(&cheap_launch);

    let business_launch = BusinessLaunchFactory.create();
    business_launch.print_prices(&business_launch);
}
