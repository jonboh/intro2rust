#![allow(unused, dead_code)]

mod adventure;
mod characters;
mod peasant;

use characters::{Adventurer, Encounter, Hero};

fn save_world(party: (Hero, Hero, Hero)) {
    todo!()
}

fn main() {
    let playable_character = Adventurer::new("Aria Lion");
    let first_companion = Adventurer::new("Varik Morrowfell");
    let second_companion = Adventurer::new("Evelyn Starwhiser");

    // NOTE: level_up consumes the adventurer
    let playable_character : Hero = playable_character.level_up(Encounter::Castle);
    let first_companion = first_companion.level_up(Encounter::RuinedCursedCastle);
    let second_companion = second_companion.level_up(Encounter::Merlin);

    save_world((playable_character, first_companion, second_companion));
}
