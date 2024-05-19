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







fn multiple_mut_references() {
    let mut my_contraption = Contraption {
        lights_on: 1729,
        active_mechanisms: 42,
    };

    let view = &mut my_contraption;
    let other_view = &mut my_contraption;

    do_things_mutably_borrowing(view);

    do_things_mutably_borrowing(other_view);

}






