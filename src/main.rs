use std::collections::VecDeque;

use colored::Colorize;
use termsize::Size;

struct Field {
    name: String,
    size: Size,
    objects: Vec<Object>,
}

impl Field {
    fn new(name: String, objects: Vec<Object>) -> Self {
        let terminal_size = termsize::get().unwrap();
        Field {
            name,
            size: terminal_size, // Rewrite it to tuple later. It will allow more versatile approaches.
            objects,
        }
    }

    fn render(&self) {
        /*
         * Better aproach will be possible if:
         * 1. Program will create array with size of field.
         * 2. Reduse amount of for loops lol.
         */
        let cols = self.size.cols as usize;
        let rows = self.size.rows as usize;
        let mut render_size: Vec<Vec<char>> = vec![vec!['-'; cols]; rows];
        let mut rendered: String = Default::default();
        for object in &self.objects {
            // This probably will create issues in future. But isize allow substraction.
            render_size[object.position.0 as usize][object.position.1 as usize] = object.draw();
        }
        for row in 0..rows {
            for col in 0..cols {
                rendered.push(render_size[row][col]);
            }
            rendered.push('\n');
        }
        println!("{}", rendered);
    }

    fn resize(&mut self) {
        let terminal_size = termsize::get().unwrap();
        self.size = terminal_size;
    }

    fn camera_move(&mut self, direction: MovementDirection, move_amount: isize) {
        let mut diff_y: isize = 0;
        let mut diff_x: isize = 0;
        match direction {
            MovementDirection::Up => diff_y = move_amount,
            MovementDirection::Left => diff_x = move_amount,
            MovementDirection::Right => diff_x = -move_amount,
            MovementDirection::Down => diff_y = -move_amount,
            _ => {}
        }
        for object in &mut self.objects {
            object.position.0 += diff_y;
            object.position.1 += diff_x;
            dbg!(object.position.0);
            dbg!(object.position.1);
        }
    }
}

enum MovementDirection {
    Up,
    Left,
    Right,
    Down,
}

struct Object {
    name: String,
    position: (isize, isize),
}

impl Drawable for Object {
    // Later I should define it on object level basis. It's WIP stuff.
    fn draw(&self) -> char {
        '0'
    }
}

struct AnimatedObject {
    name: String,
    position: (u16, u16),
    animation: Vec<Sprite>,
}

struct Sprite {
    image: Vec<char>,
}

impl Sprite {
    fn new(char_array: &[char]) -> Self {
        let image = char_array.to_vec();
        Sprite { image }
    }
}

trait Drawable {
    fn draw(&self) -> char;
}

fn main() {
    println!("Hello, world!");
    let test = Object {
        name: "test".to_string(),
        position: (10, 30),
    };
    let test2 = Object {
        name: "test2".to_string(),
        position: (10, 50),
    };
    let test3 = Object {
        name: "test3".to_string(),
        position: (10, 40),
    };
    let mut vector_objects = Vec::new();
    vector_objects.push(test);
    vector_objects.push(test2);
    vector_objects.push(test3);
    let mut a = Field::new("main_field".to_string(), vector_objects);
    let mut buf = String::default();
    loop {
        a.render();
        std::io::stdin().read_line(&mut buf).unwrap();
        match buf.trim() {
            "w" => a.camera_move(MovementDirection::Up, 1),
            "s" => a.camera_move(MovementDirection::Down, 1),
            "a" => a.camera_move(MovementDirection::Left, 4),
            "d" => a.camera_move(MovementDirection::Right, 4),
            _ => {}
        }
        buf.clear();
    }
}
