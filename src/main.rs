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
        for curr_row in 0..self.size.rows {
            for curr_col in 0..self.size.cols {
                for object in &self.objects {
                    if ((object.position.0 == curr_row) && (object.position.1 == curr_col)) {
                        object.draw();
                    } else {
                        print!("{}", "&".bright_black());
                    }
                }
            }
            println!();
        }
    }

    fn resize(&mut self) {
        let terminal_size = termsize::get().unwrap();
        self.size = terminal_size;
    }
}

struct Object {
    name: String,
    position: (u16, u16),
}

impl Drawable for Object {
    fn draw(&self) {
        print!("0");
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
    fn draw(&self);
}

fn main() {
    println!("Hello, world!");
    let test = Object {
        name: "test".to_string(),
        position: (10, 10),
    };
    let mut vector_objects = Vec::new();
    vector_objects.push(test);
    let a = Field::new("main_field".to_string(), vector_objects);

    loop {
        a.render();
    }
}
