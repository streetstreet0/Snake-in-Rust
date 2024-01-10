use std::collections::HashMap;
use std::io;
use rand::Rng;
use rand::seq::SliceRandom;
use std::thread;
use std::time::Duration;

fn main() {
    // const WIDTH: i32 = 34;
    // const HEIGHT: i32 = 13;
    const WIDTH: i32 = 5;
    const HEIGHT: i32 = 5;

    let mut snake = generate_initial_snake(WIDTH, HEIGHT);
    let mut food = match generate_food(&snake, WIDTH, HEIGHT){
        Some(food_item) => food_item,
        None => {
            println!("Error: initial food was not generated!");
            return;
        }
    };
    draw_screen(&snake, &food, WIDTH, HEIGHT);

    loop { 
        // let new_direction = Direction::random_direction();
        let new_direction = text_input_direction(&snake);
        snake.change_direction(new_direction);

        snake.move_snake(&food);
        if food.is_on_snake(&snake) {
            food = match generate_food(&snake, WIDTH, HEIGHT) {
                Some(food_item) => food_item,
                None => {
                    println!("VICTORY!");
                    return;
                },
            };
        }
        

        draw_screen(&snake, &food, WIDTH, HEIGHT);

        if snake.lost_game(WIDTH, HEIGHT) {
            println!("GAME OVER!");
            let size = snake.size;
            println!("Your Final Score was {size}");
            break;
        }

        // let duration = Duration::from_millis(500);
        // thread::sleep(duration);
    }
}

fn text_input_direction(snake: &Snake) -> Direction {
    let mut movement = String::new();
    io::stdin().read_line(&mut movement).expect("Failed to read line");
    match movement.trim().chars().next() {
        Some(char) => match char {
            'a' => Direction::Left,
            's' => Direction::Down,
            'd' => Direction::Right,
            'w' => Direction::Up,
            _ => snake.direction,
        },
        None => snake.direction,
    }
}

// food is just a coordinate
fn generate_food(snake: &Snake, width: i32, height: i32) -> Option<Food> {
    let max_size: usize = (width as usize) * (height as usize) * 3 / 10;
    
    // if the snake gets too large, we change to a new algorithm
    if snake.size < max_size {
        let mut rand_x = rand::thread_rng().gen_range(0..=(width-1));
        let mut rand_y = rand::thread_rng().gen_range(0..=(height-1));
        let mut food = Food {
        coord: Coordinate {
            x: rand_x,
            y: rand_y,
            }
        };

        while food.is_on_snake(snake) {
            rand_x = rand::thread_rng().gen_range(0..=(width-1));
            rand_y = rand::thread_rng().gen_range(0..=(height-1));
            food.coord = Coordinate {
                x: rand_x,
                y: rand_y,
            };
        }
        
        Some(food)
    }
    else if snake.size == (width as usize) * (height as usize) {
        return None
    }
    else {
        let mut valid_coords = HashMap::new();
        let mut x_counter = 0;
        while x_counter < width {
            let mut y_counter = 0;
            while y_counter < height {
                let coord = Coordinate {
                    x: x_counter,
                    y: y_counter,
                };
                valid_coords.insert((x_counter, y_counter), coord);
                y_counter += 1;
            }
            x_counter += 1;
        }

        for snake_part in &snake.parts {
            valid_coords.remove(&(snake_part.coord.x, snake_part.coord.y));
        }

        let valid_coord_values: Vec<Coordinate> = valid_coords.values().cloned().collect();
        match valid_coord_values.choose(&mut rand::thread_rng()) {
            Some(rand_coord) => Some(Food {
                coord: *rand_coord,
            }),
            None => {
                println!("an error has occured in generating a random key");
                None
            },
        }
    }
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
            let x = snake_part.coord.x;
            match entities_in_line.get(&x) {
                Some(entity) => {
                    if snake_part.is_head {
                        entities_in_line.insert(x, Entity::SnakePart(String::from(snake_part.symbol())));
                    }
                    else {
                        match entity {
                            Entity::Space => {
                                entities_in_line.insert(x, Entity::SnakePart(String::from(snake_part.symbol())));
                            },
                            _ => (),
                        }
                    }
                },
                None => {
                    entities_in_line.insert(x, Entity::SnakePart(String::from(snake_part.symbol())));
                },
            }
            
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

    fn ate_itself(&self) -> bool {
        let head = &self.parts[0];

        let mut counter = 1;
        while counter < self.parts.len() {
            let current_part = &self.parts[counter];
            if current_part.coord == head.coord {
                return true;
            }
            counter += 1;
        }
        false
    }

    fn lost_game(&self, width: i32, height: i32) -> bool {
        self.is_off_screen(width, height)|| self.ate_itself()
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