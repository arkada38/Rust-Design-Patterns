use design_patterns::behavioral::chain_of_responsibility::police_department::{
    Detective, Patrolman, Policeman,
};

// Let's create a chain of policemen (Jack -> Tom -> Chuck).
// Every officer has an individual level of deduction.
// And every crime has a difficult lvl.
// Officer can investigate the crime by himself if his deduction is not less
// than crime's difficulty lvl.
// The officer passes the crime's case if the crime's difficulty lvl bigger
// than the officer's deduction.
fn main() {
    let chuck = Detective::new(8, "Chuck");

    let mut tom = Detective::new(5, "Tom");
    tom.set_next(Box::new(chuck));

    let mut jack = Patrolman::new(2, "Jack");
    jack.set_next(Box::new(tom));

    for crime_lvl in (1..=10u8).step_by(3) {
        println!("Investigation of a new crime (lvl: {crime_lvl})");
        jack.investigate(crime_lvl);
        println!();
    }
}
