struct Peasant {
    name: String
}


enum Encounter {
    Castle,
    RuinedCursedCastle,
    Merlin,
    Troll // <- Phil: it would be cool to meet a troll!
}


impl Peasant {
    fn visit(&self, encounter: Encounter) {
        match encounter {
            Encounter::Castle => self.pay_taxes(),
            Encounter::RuinedCursedCastle => self.run_for_your_life(),
            Encounter::Merlin => self.ask_for_luck_potion(),
        }
    }

    fn pay_taxes(&self) {
        todo!()
    }

    fn run_for_your_life(&self) {
        todo!()
    }

    fn ask_for_luck_potion(&self) {
        todo!()
    }
}


