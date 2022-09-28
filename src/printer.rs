pub struct term {
    width: i32,
    height: i32
}

impl term {
    pub fn new() {
        termsize::get().map(|size| {
            print!("{} {}", size.rows, size.cols)
        });
    }

}