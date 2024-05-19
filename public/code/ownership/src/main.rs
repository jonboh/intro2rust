#![allow(unused)]


struct Contraption {
    lights_on: u32,
    active_mechanisms: u16,
}

fn do_things_borrowing(c: &Contraption) {
    todo!() // cannot change c
}
fn do_things_mutably_borrowing(c: &mut Contraption) {
    todo!()
}
fn do_things_moving(c: Contraption) {
    todo!() // c will be destroyed when exiting the scope
}

fn main() {
    let mut my_contraption = Contraption {
        lights_on: 1729,
        active_mechanisms: 42,
    };

    do_things_borrowing(&my_contraption);

    do_things_mutably_borrowing(&mut my_contraption);

    do_things_moving(my_contraption);
}







fn use_after_move() {
    let mut my_contraption = Contraption {
        lights_on: 1729,
        active_mechanisms: 42,
    };

    do_things_moving(my_contraption);

    my_contraption.active_mechanisms = 7;
}






