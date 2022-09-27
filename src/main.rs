mod rubiks;

use rubiks::Rubiks;

fn main() {
    Rubiks::testing();

    let mut cube = Rubiks::new();

    println!("{:?}" , cube.get_pos());
}
