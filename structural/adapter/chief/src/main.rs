extern crate chief;

use chief::Chief;
use chief::chief_adapter::ChiefAdapter;

fn main() {
    let chief = ChiefAdapter::new();
    let price = chief.get_cost();
    chief.make_dinner();
    chief.take_money(price + 5);
}
