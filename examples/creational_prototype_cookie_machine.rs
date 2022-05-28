use design_patterns::creational::prototype::cookie_machine::{Cookie, CookieMachine};

fn main() {
    let coconut_cookie = Cookie {
        name: "coconut",
        weight: 16,
    };
    let coconut_cookie_machine = CookieMachine::new(coconut_cookie);
    let cloned_cookie = coconut_cookie_machine.make_cookie();
    println!(
        "{} clone with weight {}\n",
        cloned_cookie.name, cloned_cookie.weight
    );

    let chocolate_cookie = Cookie {
        name: "chocolate",
        weight: 28,
    };
    let chocolate_cookie_machine = CookieMachine::new(chocolate_cookie);
    let chocolate_cookies = vec![chocolate_cookie_machine.make_cookie(); 6];

    for cookie in chocolate_cookies {
        println!("{} clone with weight {}", cookie.name, cookie.weight);
    }
}
