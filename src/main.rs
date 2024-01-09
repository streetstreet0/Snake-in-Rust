use std::collections::HashMap;

fn main() {
    const WIDTH: usize = 34;
    const HEIGHT: usize = 13;

    let mut snake = generate_initial_snake(WIDTH, HEIGHT);
    
    draw_screen(&snake, WIDTH, HEIGHT);
}

fn draw_screen(snake: &Snake, width: usize, height: usize) {
    draw_top_bottom_line(width);

    let mut line_num = 0;
    while line_num < height {
        draw_centre_line(snake, width, line_num);
        line_num += 1;
    }

    draw_top_bottom_line(width);
}

fn draw_centre_line(snake: &Snake, width: usize, line: usize) {
    let mut entities_in_line = HashMap::new();
    let mut position: usize = 0;
    while position < width {
        entities_in_line.insert(position, Entity::Space);
        position += 1;
    }

    for snake_part in &snake.parts {
        if snake_part.coord.y == line {
            entities_in_line.insert(snake_part.coord.x, Entity::SnakePart);
        }
    }

    let mut line = String::from("|");
    let mut counter: usize = 0;
    while counter < width {
        match entities_in_line.get(&counter) {
            Some(Entity::SnakePart) => line.push_str("O"),
            Some(Entity::Food) => line.push_str("X"),
            Some(Entity::Space) => line.push_str(" "),
            _ => println!("An Error has occured!"),
        }
        counter += 1    
    }
    line.push_str("|");

    println!("{line}");
}

fn draw_top_bottom_line(width: usize) {
    let mut line = String::from("+");
    let mut counter = 0;
    while counter < width {
        line.push_str("-");
        counter += 1;
    }
    line.push_str("+");

    println!("{line}");
}

fn generate_initial_snake(width: usize, height: usize) -> Snake {
    let y_position = if height % 2 == 1 {height/2} else {height/2 - 1};

    let snake_tail = SnakePart {
        coord: Coordinate {
            x: width-1,
            y: y_position,
        },
        direction: Direction::Left,
    };

    let snake_head = SnakePart {
        coord: Coordinate {
            x: width-2,
            y: y_position,
        },
        direction: Direction::Left,
    };

    let snake_parts = vec![snake_head, snake_tail];
    
    Snake {
        parts: snake_parts,
        size: 2,
        direction: Direction::Left,
    }
}

#[derive(Debug)]
struct SnakePart {
    coord: Coordinate,
    direction: Direction,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
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
    // note: the first item in the vector is the head
    parts: Vec<SnakePart>,
    size: usize,
    direction: Direction,
}

impl Snake {
    fn move_snake(&mut self, growing: bool) {
        let direction = &self.direction;

        let mut old_head = self.parts.remove(0);

        let mut new_head = SnakePart {
            coord: old_head.coord.increment(&direction),
            direction: direction.clone(),
        };

        let mut new_parts = vec![new_head, old_head];
        while self.parts.len() > 0 {
            new_parts.push(self.parts.remove(0));
        }
        self.parts = new_parts;

        if !growing {
            self.parts.remove(self.size-1);
        }
        else {
            self.size += 1;
        }

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

enum Entity {
    SnakePart,
    Food,
    Space,
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