# Cohort

Sample project with *Command*

## Structure

- Cohort
- Command (*trait*)
  - AttackCommand
  - RetreatCommand
- CohortCommander

## Implementation

```rust
struct Cohort {}

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

struct AttackCommand {}

impl Command for AttackCommand {
    fn execute(&self, cohort: &Cohort) {
        cohort.move_to("forward".to_string());
    }

    fn unexecute(&self, cohort: &Cohort) {
        cohort.stop();
    }
}

struct RetreatCommand {}

impl Command for RetreatCommand {
    fn execute(&self, cohort: &Cohort) {
        cohort.move_to("back".to_string());
    }

    fn unexecute(&self, cohort: &Cohort) {
        cohort.stop();
    }
}

enum CurrentCommand {
    Attack,
    Retreat
}

struct CohortCommander {
    cohort: Cohort,
    attack_command: AttackCommand,
    retreat_command: RetreatCommand,
    current_command: Option<CurrentCommand>
}

impl CohortCommander {
    fn new(cohort: Cohort, attack_command: AttackCommand, retreat_command: RetreatCommand) -> CohortCommander {
        CohortCommander { cohort, attack_command, retreat_command, current_command: None }
    }

    fn attack(&mut self) {
        self.current_command = Some(CurrentCommand::Attack);
        self.attack_command.execute(&self.cohort);
    }

    fn retreat(&mut self) {
        self.current_command = Some(CurrentCommand::Retreat);
        self.retreat_command.execute(&self.cohort);
    }

    fn stop(&self) {
        match self.current_command {
            Some(CurrentCommand::Attack) => self.attack_command.unexecute(&self.cohort),
            Some(CurrentCommand::Retreat) => self.retreat_command.unexecute(&self.cohort),
            None => println!("The cohort is already stoping"),
        }
    }
}

fn main() {
    let mut commander = CohortCommander::new(Cohort{}, AttackCommand{}, RetreatCommand{});
    commander.attack();
    commander.stop();
    commander.retreat();
    commander.stop();
}
```

### output

```bash
The cohort moves to forward
The cohort stops
The cohort moves to back
The cohort stops
```
