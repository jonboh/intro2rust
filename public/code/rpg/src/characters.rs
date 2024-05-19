use crate::adventure::{Condition, Monster};

pub struct Adventurer {
    name: String,
}

pub struct Soldier {
    name: String,
    health: i32,
}

pub struct Wizard {
    name: String,
    health: i32,
    school: WizardSchool,
}

enum WizardSchool {
    Conjuration,
    Necromancy,
}

pub enum Hero {
    Soldier(Soldier),
    Wizard(Wizard),
}

pub enum Encounter {
    Castle,
    RuinedCursedCastle,
    Merlin,
}

impl Adventurer {
    pub fn new(name: &str) -> Adventurer {
        Adventurer {
            name: name.to_string(),
        }
    }

    pub fn level_up(self, encounter: Encounter) -> Hero {
        match encounter {
            Encounter::Castle => Hero::Soldier(Soldier {
                name: self.name,
                health: 100
            }),
            Encounter::RuinedCursedCastle => Hero::Wizard(Wizard {
                name: self.name,
                health: 80,
                school: WizardSchool::Necromancy,
            }),
            Encounter::Merlin => Hero::Wizard(Wizard {
                name: self.name,
                health: 70,
                school: WizardSchool::Conjuration,
            }),
        }
    }
}

impl Soldier {
    fn slash< T: Monster >(&self, enemy: &mut T) {
        enemy.take_damage(5);
    }
}

impl Wizard {
    fn enchant< T: Monster >(&self, enemy: &mut T) {
        enemy.take_damage(3);
        match self.school {
            WizardSchool::Conjuration => enemy.apply_condition(Condition::Burning),
            WizardSchool::Necromancy => enemy.apply_condition(Condition::Cursed),
        }
    }
}

impl Hero {
    fn attack< T: Monster >(&self, enemy: &mut T) {
        match self {
            Hero::Soldier(soldier) => soldier.slash(enemy),
            Hero::Wizard(wizard) => wizard.enchant(enemy),
        }
    }
}
