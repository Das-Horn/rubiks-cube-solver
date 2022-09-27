pub struct Rubiks {
    pos: Vec<Vec<i32>>,
    
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

        return Rubiks { pos: final_pos };
    }

    // Printing Functions

    pub fn print_layout(&mut self){
        let mut count = 1;

        for i in 0..self.pos.len(){
            for j in 0..self.pos[i].len() {
                print!("{}", self.pos[i][j]);
                if (count % 3) == 0 {
                    print!("\n");
                }
                count += 1;
            }
            print!("\n");
        }
    }

    // Getters

    pub fn get_pos(&mut self) -> &Vec<Vec<i32>> {
        return &self.pos;
    }
}