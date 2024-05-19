enum NPCAttitude {
    Friendly,
    Neutral,
    Enemy,
}

fn react(attitude: NPCAttitude) {
    match attitude {
        NPCAttitude::Friendly => {
            println!("Good to sea you adventurer!")
        }
        NPCAttitude::Neutral => {
            println!("Who are you?")
        }
        NPCAttitude::Enemy => {
            println!("Prepare to die!")
        }
    }
}

fn main() {
    todo!();
}
