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
            render_size[object.position.0][object.position.1] = object.draw();
        }
        for row in 0..rows {
            for col in 0..cols {
                //print!("{}", render_size[row][col]);
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
}

struct Object {
    name: String,
    position: (usize, usize),
}

impl Drawable for Object {
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
        position: (5, 30),
    };
    let mut vector_objects = Vec::new();
    vector_objects.push(test);
    let a = Field::new("main_field".to_string(), vector_objects);

    loop {
        a.render();
    }
}
