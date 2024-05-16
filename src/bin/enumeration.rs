enum Move {
    Left,
    Right,
    Up,
    Down,
}

struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    fn change_position (&mut self, new_pos: Move) {
        match new_pos {
            Move::Left => self.x = self.x - 1,
            Move::Right => self.x = self.x + 1,
            Move::Up => self.y = self.y + 1,
            Move::Down => self.y = self.y - 1,
        }  
    }
}

fn main() {
    let mut new_coord = Coordinate {x: 5, y: 5};

    println!("Current Coordinate: ({}, {})", new_coord.x, new_coord.y);
    
    new_coord.change_position(Move::Left);
    println!("Position moved to left!");
    println!("Current Coordinate: ({}, {})", new_coord.x, new_coord.y);

    new_coord.change_position(Move::Right);
    println!("Postion moved to right!");
    println!("Current Coordinate: ({}, {})", new_coord.x, new_coord.y);

    new_coord.change_position(Move::Up);
    println!("Position moved up!");
    println!("Current Coordinate: ({}, {})", new_coord.x, new_coord.y);

    new_coord.change_position(Move::Down);
    println!("Position moved down!");
    println!("Current Coordinate: ({}, {})", new_coord.x, new_coord.y);
}