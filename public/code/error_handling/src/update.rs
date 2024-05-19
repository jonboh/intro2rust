fn fetch_update() -> Option< UpdateMetadata > {
    todo!()
}

fn update_game() {
    match fetch_update() {
        Some(update) => todo!(),
        None => println!("There are no updates available")
    }
}



struct UpdateMetadata {}
