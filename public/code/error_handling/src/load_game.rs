#![allow(unused)]

struct GameState {}

struct OldSave {}

struct Save {}


impl Save {
    fn open(filename: String) ->Result< Save, ErrorKind > {
        todo!()
    }

    fn update_save_and_load(old: OldSave) -> Save{
        todo!()
    }

    fn generate_new_game() -> Save {
        todo!()
    }
}

enum ErrorKind {
    NotFound,
    OldGameVersion(OldSave),
    NewGameVersion,
    CorruptedSave,
}

fn load_latest_save(savespath: String) -> GameState {
    let game_state = match Save::open(savespath) {
        Ok(save) => save,
        Err(error) => match error {
            ErrorKind::NotFound => todo!(),
            ErrorKind::OldGameVersion(old_save) => todo!(),
            ErrorKind::NewGameVersion => todo!(),
            ErrorKind::CorruptedSave => panic!("This file does not look like a savefile. Aborting"),
        },
    };

    todo!()
}










