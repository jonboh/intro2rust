void do_things_borrowing(const Contraption& c);

void do_things_mutably_borrowing(Contraption& c);

void do_things_moving(Contraption c);






int main () {

    // ...

    do_things_borrowing(my_contraption);

    do_things_mutably_borrowing(my_contraption);

    // ! this will call the copy constructor!!
    do_things_moving(my_contraption); 

    do_things_moving(std::move(my_contraption));
}
