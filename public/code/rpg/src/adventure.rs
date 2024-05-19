pub enum Condition {
    None,
    Burning,
    Cursed,
    Fearful,
}

pub trait Monster {
    fn take_damage(&mut self, dmg: i32);

    fn apply_condition(&mut self, cnd: Condition);
}

struct Goblin {
    health: i32,
    state: Condition,
}

impl Monster for Goblin {
    fn take_damage(&mut self, dmg: i32) {
        self.health = self.health.saturating_sub(dmg);
        self.apply_condition(Condition::Fearful);
    }

    fn apply_condition(&mut self, cnd: Condition) {
        self.state = cnd;
    }
}

struct Dragon {
    health: i32,
    state: Condition,
}

impl Monster for Dragon {
    fn take_damage(&mut self, dmg: i32) {
        // dragons have tough skin
        let dmg = dmg.saturating_sub(2); 
        self.health = self.health.saturating_sub(dmg);
    }

    fn apply_condition(&mut self, cnd: Condition) {
        // dragons dont know fear and are immune to fire
        match cnd {
            (Condition::None | Condition::Burning | Condition::Fearful) => {
                self.state = Condition::None
            }
            Condition::Cursed => self.state = Condition::Cursed,
        }
    }
}
