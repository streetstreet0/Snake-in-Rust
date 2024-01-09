fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct SnakePart {
    coord: Coordinate,
    is_head: bool,
    direction: Direction,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn increment(&self, direction: &Direction) -> Coordinate {
        match direction {
            Direction::Up => {
                Coordinate {
                    x: self.x,
                    y: self.y+1,
                }
            }
            Direction::Down => {
                Coordinate {
                    x: self.x,
                    y: self.y-1,
                }
            }
            Direction::Left => {
                Coordinate {
                    x: self.x-1,
                    y: self.y,
                }
            }
            Direction::Right => {
                Coordinate {
                    x: self.x + 1,
                    y: self.y,
                }
            }
        }
    }
}

#[derive(Debug)]
struct Snake {
    parts: Vec<SnakePart>,
    size: usize,
    direction: Direction,
}

impl Snake {
    fn move_snake(&mut self, growing: bool) {
        if !growing {
            self.parts.remove(self.size-1);
        }
        else {
            self.size += 1;
        }
        let direction = &self.direction;

        let mut old_head = self.parts.remove(0);
        old_head.is_head = false;

        let mut new_head = SnakePart {
            is_head: true,
            coord: old_head.coord.increment(&direction),
            direction: direction.clone(),
        };

        let mut new_parts = vec![new_head, old_head];
        while self.parts.len() > 0 {
            new_parts.push(self.parts.remove(0));
        }
        self.parts = new_parts;

        if self.parts.len() != self.size {
            println!("error, sizes do not match");
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        let mut option_head = self.parts.get(0);
        match option_head {
            None => (),
            Some(head) => {
                if !direction.is_opposite(&head.direction) {
                    self.direction = direction;
                }
            }
        }
    }

    fn is_on_itself(&self, coord: Coordinate) -> bool {
        for snake_part in &self.parts {
            if snake_part.coord == coord {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn is_opposite(&self, direction: &Direction) -> bool {
        match self {
            Direction::Up => { match direction {
                Direction::Down => true,
                _ => false,
            } }
            Direction::Down => { match direction {
                Direction::Up => true,
                _ => false,
            } }
            Direction::Left => { match direction {
                Direction::Right => true,
                _ => false,
            } }
            Direction::Right => { match direction {
                Direction::Left => true,
                _ => false,
            } }
        }
    }
}

// RECURSIVE DATASTRUCTURE WILL NOT WORK WITH RUST
// #[derive(Debug)]
// struct SnakePart {
//     coord: Coordinate,
//     next: Option<Box<Self>>,
// }

// impl SnakePart {
//     fn new(coord: Coordinate) -> Self {
//         Self {
//             coord,
//             next: None,
//         }
//     }

//     fn add_next(&mut self, coord: Coordinate) {
//         match &self.next {
//             None => {
//                 let next_part = Self::new(coord);
//                 self.next = Some(Box::new(next_part));
//             }
//             Some(boxed_next_part) => {
//                 let mut next_part = boxed_next_part.add_next(coord);
//             }  
//         }
//     }
// }