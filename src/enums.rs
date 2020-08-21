/*
 * Enums
*/

#[derive(Debug)]
enum MOVEMENT {
    // Variants
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn move_avatar(m: MOVEMENT) {
    // Perform action based on info
    match m {
        MOVEMENT::UP => println!("Avatar moving {:?}", MOVEMENT::UP),
        MOVEMENT::DOWN => println!("Avatar moving {:?}", MOVEMENT::DOWN),
        MOVEMENT::LEFT => println!("Avatar moving {:?}", MOVEMENT::LEFT),
        MOVEMENT::RIGHT => println!("Avatar moving {:?}", MOVEMENT::RIGHT),
    }
}

pub fn run() {
    let avatar1 = MOVEMENT::LEFT;
    let avatar2 = MOVEMENT::UP;
    let avatar3 = MOVEMENT::RIGHT;
    let avatar4 = MOVEMENT::DOWN;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
