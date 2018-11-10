
// enums

enum Direction {
up,
down,
left,
right
}


fn main() {
let player_Direction : Direction = Direction::up;

match player_Direction {
Direction::up => println!("We are heading up"),
Direction::down => println!("We are heading down"),
Direction::left => println!("We are moving left"),
Direction::right => println!("We are moving right"),

}

}
