use std::collections::HashMap;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    const WIDTH: i32 = 34;
    const HEIGHT: i32 = 13;

    let mut snake = generate_initial_snake(WIDTH, HEIGHT);
    let mut food = generate_food(&snake, WIDTH, HEIGHT);
    draw_screen(&snake, &food, WIDTH, HEIGHT);

    loop { 
        let new_direction = Direction::random_direction();
        snake.change_direction(new_direction);
        snake.move_snake(&food);
        if food.is_on_snake(&snake) {
            food = generate_food(&snake, WIDTH, HEIGHT);
        }
        

        draw_screen(&snake, &food, WIDTH, HEIGHT);

        if snake.is_off_screen(WIDTH, HEIGHT) {
            println!("GAME OVER!");
            break;
        }

        let duration = Duration::from_millis(500);
        thread::sleep(duration);
    }
}

// food is just a coordinate
fn generate_food(snake: &Snake, width: i32, height: i32) -> Food {
    let mut rand_x = rand::thread_rng().gen_range(0..=(width-1));
    let mut rand_y = rand::thread_rng().gen_range(0..=(height-1));
    let mut food = Food {
        coord: Coordinate {
            x: rand_x,
            y: rand_y,
        },
    };

    while food.is_on_snake(snake) {
        rand_x = rand::thread_rng().gen_range(0..=(width-1));
        rand_y = rand::thread_rng().gen_range(0..=(height-1));
        food.coord = Coordinate {
            x: rand_x,
            y: rand_y,
        };
    }
    
    food
} 

fn draw_screen(snake: &Snake, food: &Food, width: i32, height: i32) {
    draw_top_bottom_line(snake, width, height, true);

    let mut line_num = 0;
    while line_num < height {
        draw_centre_line(snake, food, width, line_num);
        line_num += 1;
    }

    draw_top_bottom_line(snake, width, height, false);
}

fn draw_centre_line(snake: &Snake, food: &Food, width: i32, line: i32) {
    let mut entities_in_line = snake_parts_in_line(snake, width, line);
    if food.coord.y == line {
        entities_in_line.insert(food.coord.x, Entity::Food);
    }

    let mut line = String::from("");
    let mut counter: i32 = -1;
    while counter < width + 1 {
        match entities_in_line.get(&counter) {
            Some(Entity::SnakePart(symbol)) => line.push_str(symbol),
            Some(Entity::Food) => line.push_str("X"),
            Some(Entity::Space) => {
                if counter == -1 || counter == width {
                    line.push_str("|")
                }
                else {
                    line.push_str(" ")
                }
            },
            _ => println!("An Error has occured!"),
        }
        counter += 1    
    }

    println!("{line}");
}

fn draw_top_bottom_line(snake: &Snake, width: i32, height: i32, top: bool) {
    let line = match top {
        true => -1,
        false => height,
    };
    let entities_in_line = snake_parts_in_line(snake, width, line);

    let mut line = String::from("+");
    let mut counter = 0;
    while counter < width {
        match entities_in_line.get(&counter) {
            Some(entity) => {
                match entity {
                    Entity::SnakePart(symbol) => line.push_str(symbol),
                    Entity::Space => line.push_str("-"),
                    Entity::Food => (),
                }
            }
            None => println!("An Error has occured!"),
        }
        counter += 1;
    }
    line.push_str("+");

    println!("{line}");
}

fn snake_parts_in_line(snake: &Snake, width: i32, line: i32) -> HashMap<i32, Entity> {
    let mut entities_in_line = HashMap::new();
    let mut position: i32 = -1;
    while position <= width {
        entities_in_line.insert(position, Entity::Space);
        position += 1;
    }

    for snake_part in &snake.parts {
        if snake_part.coord.y == line {
            entities_in_line.insert(snake_part.coord.x, Entity::SnakePart(String::from(snake_part.symbol())));
        }
    }
    entities_in_line
}

fn generate_initial_snake(width: i32, height: i32) -> Snake {
    let y_position = if height % 2 == 1 {height/2} else {height/2 - 1};

    let snake_tail = SnakePart {
        coord: Coordinate {
            x: width-1,
            y: y_position,
        },
        direction: Direction::Left,
        is_head: false,
    };

    let snake_head = SnakePart {
        coord: Coordinate {
            x: width-2,
            y: y_position,
        },
        direction: Direction::Left,
        is_head: true,
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
    is_head: bool,
}

impl SnakePart {
    fn symbol(&self) -> &str {
        if self.is_head {
            match self.direction {
                Direction::Up => "^",
                Direction::Down => "v",
                Direction::Left => "<",
                Direction::Right => ">",
            }
        }
        else {
            "O"
        }
    }
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
                    y: self.y-1,
                }
            }
            Direction::Down => {
                Coordinate {
                    x: self.x,
                    y: self.y+1,
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
    fn move_snake(&mut self, food: &Food) {
        let direction = &self.direction;

        let mut old_head = self.parts.remove(0);
        old_head.is_head = false;

        let new_head = SnakePart {
            coord: old_head.coord.increment(&direction),
            direction: direction.clone(),
            is_head: true
        };

        let mut new_parts = vec![new_head, old_head];
        while self.parts.len() > 0 {
            new_parts.push(self.parts.remove(0));
        }
        self.parts = new_parts;

        if food.is_on_snake(self) {
            self.size += 1;
        }
        else {
            self.parts.remove(self.size);
        }

        if self.parts.len() != self.size {
            println!("error, sizes do not match");
        }
    }

    fn change_direction(&mut self, direction: Direction) {
        let option_head = self.parts.get(0);
        match option_head {
            None => (),
            Some(head) => {
                if !direction.is_opposite(&head.direction) {
                    self.direction = direction;
                }
            }
        }
    }

    fn is_off_screen(&self, width: i32, height: i32) -> bool {
        let snake_part = &self.parts[0];
        let x = snake_part.coord.x;
        let y = snake_part.coord.y;


        if x < 0 || x >= width || y < 0 || y >= height {
            true
        }
        else {
            false
        }
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

    fn random_direction() -> Direction {
        let rand_num = rand::thread_rng().gen_range(0..=3);
        match rand_num {
            0 => Direction::Up,
            1 => Direction::Left,
            2 => Direction::Down,
            3 => Direction::Right,
            _ => {
                println!("Error: 5th direction generated");
                Direction::Up
            },
        }
    }
}

enum Entity {
    SnakePart(String),
    Food,
    Space,
}

struct Food {
    coord: Coordinate,
}

impl Food {
    fn is_on_snake(&self, snake: &Snake) -> bool {
        for snake_part in &snake.parts {
            if snake_part.coord == self.coord {
                return true;
            }
        }
        false
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