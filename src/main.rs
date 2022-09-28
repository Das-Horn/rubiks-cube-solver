mod rubiks;
mod printer;

use rubiks::Rubiks;


fn main() {
    Rubiks::testing();

    let mut cube = Rubiks::new();

    // println!("{:?}" , cube.get_pos());
    // println!("\n\n");
    // cube.print_layout();

    cube.print_UV();

    printer::term::new();
}
