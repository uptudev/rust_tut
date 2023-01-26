mod basics;
mod guessing_game;
fn main() {
    //Basic OOP, using the basics module to import methods written in ./basics.rs
    basics::hello_world();
    guessing_game::guess();
}
