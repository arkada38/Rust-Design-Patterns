use design_patterns::behavioral::command::cohort::{
    AttackCommand, Cohort, CohortCommander, RetreatCommand,
};

fn main() {
    let mut commander = CohortCommander::new(Cohort {}, AttackCommand {}, RetreatCommand {});
    commander.attack();
    commander.stop();
    commander.retreat();
    commander.stop();
}
