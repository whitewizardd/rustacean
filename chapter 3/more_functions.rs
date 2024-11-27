

fn main() {

    let mut door = Door {};
    let mut player = Player {};

    open(&mut door);
    walk_in(&mut player, &door);
    close (&mut door);
}