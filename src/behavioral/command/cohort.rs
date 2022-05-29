pub struct Cohort {}

impl Cohort {
    fn move_to(&self, direction: String) {
        println!("The cohort moves to {}", direction);
    }

    fn stop(&self) {
        println!("The cohort stops");
    }
}

trait Command {
    fn execute(&self, cohort: &Cohort);
    fn unexecute(&self, cohort: &Cohort);
}

pub struct AttackCommand {}

impl Command for AttackCommand {
    fn execute(&self, cohort: &Cohort) {
        cohort.move_to("forward".to_string());
    }

    fn unexecute(&self, cohort: &Cohort) {
        cohort.stop();
    }
}

pub struct RetreatCommand {}

impl Command for RetreatCommand {
    fn execute(&self, cohort: &Cohort) {
        cohort.move_to("back".to_string());
    }

    fn unexecute(&self, cohort: &Cohort) {
        cohort.stop();
    }
}

pub enum CurrentCommand {
    Attack,
    Retreat,
}

pub struct CohortCommander {
    cohort: Cohort,
    attack_command: AttackCommand,
    retreat_command: RetreatCommand,
    current_command: Option<CurrentCommand>,
}

impl CohortCommander {
    pub fn new(
        cohort: Cohort,
        attack_command: AttackCommand,
        retreat_command: RetreatCommand,
    ) -> CohortCommander {
        CohortCommander {
            cohort,
            attack_command,
            retreat_command,
            current_command: None,
        }
    }

    pub fn attack(&mut self) {
        self.current_command = Some(CurrentCommand::Attack);
        self.attack_command.execute(&self.cohort);
    }

    pub fn retreat(&mut self) {
        self.current_command = Some(CurrentCommand::Retreat);
        self.retreat_command.execute(&self.cohort);
    }

    pub fn stop(&self) {
        match self.current_command {
            Some(CurrentCommand::Attack) => self.attack_command.unexecute(&self.cohort),
            Some(CurrentCommand::Retreat) => self.retreat_command.unexecute(&self.cohort),
            None => println!("The cohort is already stoping"),
        }
    }
}
