pub struct Rubiks {
    pos: Vec<Vec<i32>>,
    col: Vec<String>,
}

impl Rubiks {
    pub fn testing() {
        println!("Working");
    }
    // init method
    pub fn new() -> Rubiks {
        // Default Pos
        let mut temp_vec = Vec::new();
        let mut final_pos: Vec<Vec<i32>> = Vec::new();

        for i in 0..6 {
            for _j in 0..9 {
                match i{
                    0=>temp_vec.push(0),
                    1=>temp_vec.push(1),
                    2=>temp_vec.push(2),
                    3=>temp_vec.push(3),
                    4=>temp_vec.push(4),
                    5=>temp_vec.push(5),
                    i32::MIN..=-1_i32 | 6_i32..=i32::MAX => print!("Error")
                };
            }
            final_pos.push(temp_vec.to_vec());
            temp_vec.clear();
        }

        //Colours

        let mut default_colours = Vec::new();

        default_colours.push(String::from("\x1b[38,05,205"));

        return Rubiks { pos: final_pos,
                        col: default_colours
                    };
    }

    // Printing Functions

    pub fn print_layout(&mut self){
        let mut count = 1;

        for i in 0..self.pos.len(){
            for j in 0..self.pos[i].len() {
                print!("\x1b[38;5;205m{} ", self.pos[i][j]);
                if (count % 3) == 0 {
                    print!("\n");
                }
                count += 1;
            }
            print!("\n");
        }
        print!("\x1b[0m");
    }

    pub fn print_UV(&mut self) {
                self.print_long(4);
                self.print_short();
                self.print_long(5);
    }

    fn print_short(&mut self) {
        for i in 0..3 {
            for j in 0..12 {
                print!("{} ", self.pos[j / 3][((j % 3)) * i]);
            }
            print!("\n");
        }
    }

    fn print_long(&mut self, current_index:usize) {
        for i in 0..3 {
            print!("            ");
            print!("{} {} {} ", self.pos[current_index][i * 3 + 0],
                                self.pos[current_index][i * 3 + 1],
                                self.pos[current_index][i * 3 + 2]);
            print!("   \n");

        }
    }

    //Rotational Methods

    // Getters

    pub fn get_pos(&mut self) -> &Vec<Vec<i32>> {
        return &self.pos;
    }
}